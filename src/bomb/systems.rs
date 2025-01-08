use bevy::prelude::*;
use rand::Rng;

use crate::bomb::config::{BOMB_DEBOUNCE_SECONDS, BOMB_TIMER_SECONDS};
use crate::bomb::resources::BombDebounce;
use crate::*;

pub fn setup_bomb(mut commands: Commands) {
    commands.insert_resource(BombDebounce {
        timer: Timer::from_seconds(BOMB_DEBOUNCE_SECONDS, TimerMode::Once),
    });
}

pub fn plant_bomb_system(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut bomb_count: ResMut<BombCount>,
    mut bomb_debounce: ResMut<BombDebounce>,
    time: Res<Time>,
    player_position: Query<&Transform, With<Player>>,
    asset_server: Res<AssetServer>,
) {
    bomb_debounce.timer.tick(time.delta());

    if keyboard_input.pressed(KeyCode::Space)
        && bomb_count.value > 0
        && bomb_debounce.timer.finished()
    {
        if let Ok(transform) = player_position.get_single() {
            commands.spawn((
                PlantedBomb,
                Sprite {
                    image: asset_server.load("textures/bomb.png").clone(),
                    custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                    ..Default::default()
                },
                Transform::from_translation(Vec3::new(
                    transform.translation.x,
                    transform.translation.y,
                    0.0,
                )),
                Lifetime {
                    timer: Timer::from_seconds(BOMB_TIMER_SECONDS, TimerMode::Once),
                },
            ));
            bomb_count.value -= 1;

            let random_number = rand::thread_rng().gen_range(0..2);
            if random_number == 0 {
                commands.spawn(AudioPlayer::new(asset_server.load("audio/run.ogg")));
            } else {
                commands.spawn(AudioPlayer::new(
                    asset_server.load("audio/explosion_soon.ogg"),
                ));
            }

            bomb_debounce.timer.reset();
        }
    }
}

pub fn spawn_bombs(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    // let num = 1;

    let image_wall = asset_server.load("textures/bomb.png");
    // let x = rng.gen_range(0..=(window.width() / TILE_SIZE) as usize) * TILE_SIZE as usize;
    // let y = rng.gen_range(0..=(window.height() / TILE_SIZE) as usize) * TILE_SIZE as usize;
    commands.spawn((
        Bomb {},
        Sprite {
            image: image_wall.clone(),
            custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
            ..Default::default()
        },
        Transform::from_translation(Vec3::new(
            (TILE_SIZE * -3.0) + window.width() / 2.0,
            TILE_SIZE * 5.0 + window.height() / 2.0,
            0.0,
        )),
    ));
    commands.spawn((
        Bomb {},
        Sprite {
            image: image_wall.clone(),
            custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
            ..Default::default()
        },
        Transform::from_translation(Vec3::new(
            (-TILE_SIZE * 2.0) + window.width() / 2.0,
            TILE_SIZE * 5.0 + window.height() / 2.0,
            0.0,
        )),
    ));
    commands.spawn((
        Bomb {},
        Sprite {
            image: image_wall.clone(),
            custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
            ..Default::default()
        },
        Transform::from_translation(Vec3::new(
            (TILE_SIZE * 1.0) + window.width() / 2.0,
            TILE_SIZE * 5.0 + window.height() / 2.0,
            0.0,
        )),
    ));
    commands.spawn((
        Bomb {},
        Sprite {
            image: image_wall.clone(),
            custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
            ..Default::default()
        },
        Transform::from_translation(Vec3::new(
            (TILE_SIZE * 2.0) + window.width() / 2.0,
            TILE_SIZE * 5.0 + window.height() / 2.0,
            0.0,
        )),
    ));
    commands.spawn((
        Bomb {},
        Sprite {
            image: image_wall.clone(),
            custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
            ..Default::default()
        },
        Transform::from_translation(Vec3::new(
            (TILE_SIZE * 7.0) + window.width() / 2.0,
            TILE_SIZE * 5.0 + window.height() / 2.0,
            0.0,
        )),
    ));
}
