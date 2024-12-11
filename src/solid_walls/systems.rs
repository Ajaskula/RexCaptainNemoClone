use bevy::prelude::*;
use bevy::window::PrimaryWindow;
// use super::systems::*;
use crate::*;

pub fn spawn_solid_walls_h(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>
) {

    let window = window_query.get_single().unwrap();
    for x in (0.. 2 * window.width() as usize + TILE_SIZE as usize).step_by(TILE_SIZE as usize) {
        let solid_wall_image = asset_server.load("textures/solid_wall.png");
        commands.spawn((
            Sprite {
                image:solid_wall_image.clone(),
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                ..Default::default()
            },
            Transform::from_translation(Vec3::new(x as f32 - window.width(), -window.height(), 0.0)),
            GlobalTransform::default(),
            NotPassableForEnemy,
            NotPassableForPlayer,
        )
        );
        commands.spawn((
            Sprite {
                image:solid_wall_image.clone(),
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                ..Default::default()
            },
            Transform::from_translation(Vec3::new(x as f32 - window.width(), window.height(), 0.0)),
            GlobalTransform::default(),
            NotPassableForEnemy,
            NotPassableForPlayer
        )
        );
        
    }
}

pub fn spawn_solid_walls_v(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>
) {

    let window = window_query.get_single().unwrap();
    for y in (0.. 2 * window.height() as usize + TILE_SIZE as usize).step_by(TILE_SIZE as usize) {
        let solid_wall_image = asset_server.load("textures/solid_wall.png");
        commands.spawn((
            Sprite {
                image:solid_wall_image.clone(),
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                ..Default::default()
            },
            Transform::from_translation(Vec3::new(window.width(), y as f32 - window.height(), 0.0)),
            GlobalTransform::default(),
            NotPassableForEnemy,
            NotPassableForPlayer
        )
        );
        commands.spawn((
            Sprite {
                image:solid_wall_image.clone(),
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                ..Default::default()
            },
            Transform::from_translation(Vec3::new(-window.width(), y as f32 - window.height(), 0.0)),
            GlobalTransform::default(),
            NotPassableForEnemy,
            NotPassableForPlayer
        )
        );
        
    }
}