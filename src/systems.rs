use bevy::prelude::*;
use bevy::window::PrimaryWindow;

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
