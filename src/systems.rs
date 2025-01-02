use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_audio::PlaybackMode;
use crate::player::components::Player;
use crate::components::Lifetime;
use crate::components::Explodable;

// rozmiar kafelka mapy
pub const TILE_SIZE: f32 = 40.0;

pub fn spawn_camera(
    mut commands : Commands,
) { 
    commands.spawn(
        (Camera2d,
    )
    );
}


pub fn set_background(
    mut commands : Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>
) {

    let window = window_query.get_single().unwrap();
    for x in (0..2 * window.width() as usize).step_by(TILE_SIZE as usize) {
        for y in (0..2 * window.height() as usize).step_by(TILE_SIZE as usize) {

            let empty_tile_image: Handle<Image> = asset_server.load("textures/empty.png");
            commands.spawn(
                (Sprite {
                    image: empty_tile_image,
                    custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                    ..Default::default()
                },
                Transform {
                    translation: Vec3::new(x as f32 - window.width(), y as f32 - window.height(), -1.0),
                    ..Default::default()
                }
            )
            );
        }
    }

}



pub fn play_background_music(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) { 
    commands.spawn((
        AudioPlayer::new(
            asset_server.load("audio/egipt.ogg"),
        ),
    
        PlaybackSettings {
            mode: PlaybackMode::Loop,
            ..Default::default()
        }
    )
    );
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


// Odtwarzanie muzyki z ustawieniami odtwarzania
// audio_player.play(music_handle.clone(), playback_settings);