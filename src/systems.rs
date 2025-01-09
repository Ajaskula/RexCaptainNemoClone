use crate::bomb::components::{Bomb, ExplosionVisual};
use crate::bomb::config::{BOMB_TIMER_CHAIN_REACTION_SECONDS, EXPLOSION_RANGE};
use crate::components::Explodable;
use crate::components::Lifetime;
use crate::components::PlantedBomb;
use crate::player::components::Player;
use crate::NotPassableForEnemy;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_audio::PlaybackMode;

// rozmiar kafelka mapy
pub const TILE_SIZE: f32 = 40.0;
pub const WINDOW_HEIGHT: f32 = 40.0 * TILE_SIZE;
pub const WINDOW_WIDTH: f32 = 40.0 * TILE_SIZE;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2d,));
}

pub fn spawn_exit(mut commands: Commands, asset_server: Res<AssetServer>) {
    let image = asset_server.load("textures/gate.png");
    commands.spawn((
        Sprite {
            image,
            custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
            ..Default::default()
        },
        Transform::from_translation(Vec3::new(17.0 * TILE_SIZE, 17.0 * TILE_SIZE, 1.0)),
        NotPassableForEnemy,
    ));
}

pub fn set_background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    for x in (0..2 * WINDOW_WIDTH as usize).step_by(TILE_SIZE as usize) {
        for y in (0..2 * WINDOW_HEIGHT as usize).step_by(TILE_SIZE as usize) {
            let empty_tile_image: Handle<Image> = asset_server.load("textures/empty.png");
            commands.spawn((
                Sprite {
                    image: empty_tile_image,
                    custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                    ..Default::default()
                },
                Transform {
                    translation: Vec3::new(
                        x as f32 - WINDOW_WIDTH,
                        y as f32 - WINDOW_HEIGHT,
                        -1.0,
                    ),
                    ..Default::default()
                },
            ));
        }
    }
}

pub fn play_background_music(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        AudioPlayer::new(asset_server.load("audio/egipt.ogg")),
        PlaybackSettings {
            mode: PlaybackMode::Loop,
            ..Default::default()
        },
    ));
}

pub fn update_camera(
    mut camera: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
    player: Query<&Transform, (With<Player>, Without<Camera2d>)>,
    time: Res<Time>,
) {
    let Ok(mut camera) = camera.get_single_mut() else {
        return;
    };

    let Ok(player) = player.get_single() else {
        return;
    };

    let Vec3 { x, y, .. } = player.translation;
    let direction = Vec3::new(x, y, camera.translation.z);

    // Applies a smooth effect to camera movement using stable interpolation
    // between the camera position and the player position on the x and y axes.
    camera
        .translation
        .smooth_nudge(&direction, 2.0, time.delta_secs());
}

pub fn explodable_lifetime_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Lifetime, &Explodable)>, // Dodajemy `Explodable` do query
) {
    for (entity, mut lifetime, _explodable) in query.iter_mut() {
        // Zmniejsz czas życia
        lifetime.timer.tick(time.delta());
        if lifetime.timer.finished() {
            // Usuń element, gdy czas się skończy
            commands.entity(entity).despawn();
            println!("Wysadzam wysadzalny element {:?}", entity);
        }
    }
}

pub fn remove_explosion_sprite(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Lifetime), With<ExplosionVisual>>,
) {
    for (entity, mut lifetime) in &mut query {
        lifetime.timer.tick(time.delta());
        if lifetime.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}

pub fn explosive_lifetime_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut planted_bombs: Query<(Entity, &mut Lifetime, &Transform), With<PlantedBomb>>,
    mut explodables: Query<(Entity, &Transform), With<Explodable>>,
    mut entities: Query<(Entity, &Transform)>,
    mut bombs: Query<(Entity, &Transform), With<Bomb>>,
) {
    // przechodzę przez podłożone bomby
    for (planted_bomb_entity, mut planted_bomb_lifetime, planted_bomb_transform) in
        planted_bombs.iter_mut()
    {
        // Zmniejszam czas ich życia
        planted_bomb_lifetime.timer.tick(time.delta());

        // Jeśli bomba powinna wybuchnąć
        if planted_bomb_lifetime.timer.finished() {
            // despawnuje pombę
            commands.entity(planted_bomb_entity).despawn();
            // println!("Wybucham bombę!");

            // znajduje wszystkie elementy w zasięgu
            let explosion_range = EXPLOSION_RANGE * TILE_SIZE;

            for (_, radius_transform) in &mut entities {
                if planted_bomb_transform
                    .translation
                    .distance(radius_transform.translation)
                    <= explosion_range
                {
                    commands.spawn((
                        ExplosionVisual,
                        Sprite {
                            image: asset_server.load("textures/explosion.png"),
                            custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                            ..Default::default()
                        },
                        Lifetime {
                            timer: Timer::from_seconds(1.0, Default::default()),
                        },
                        Transform::from_translation(Vec3::new(
                            radius_transform.translation.x,
                            radius_transform.translation.y,
                            2.0,
                        )),
                    ));
                }
            }

            // przechodzę przez wysadzalne elementy w otoczeniu
            for (explodable_entity, explodable_transform) in explodables.iter_mut() {
                let distance = planted_bomb_transform
                    .translation
                    .distance(explodable_transform.translation);

                // jeśli są one wystarczająco blisko, to do konkrentej encji dodaje lifetime
                if distance <= explosion_range {
                    // Dodaj `Lifetime` do eksplodowalnych elementów
                    commands.entity(explodable_entity).insert(Lifetime {
                        timer: Timer::from_seconds(0.2, TimerMode::Once), // Dajemy im np. 2 sekundy istnienia
                    });
                }
            }

            for (bomb_entity, bomb_transform) in &mut bombs {
                if planted_bomb_transform
                    .translation
                    .distance(bomb_transform.translation)
                    <= explosion_range
                {
                    commands.spawn((
                        Sprite {
                            image: asset_server.load("textures/bomb.png").clone(),
                            custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                            ..Default::default()
                        },
                        *bomb_transform,
                        PlantedBomb,
                        Lifetime {
                            timer: Timer::from_seconds(
                                BOMB_TIMER_CHAIN_REACTION_SECONDS,
                                Default::default(),
                            ),
                        },
                    ));
                    commands.entity(bomb_entity).despawn();
                }
            }
        }
    }
}
