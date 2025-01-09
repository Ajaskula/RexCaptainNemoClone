use crate::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

fn create_wall_sprite(
    image: Handle<Image>,
    position: Vec3,
) -> (
    Sprite,
    Transform,
    Explodable,
    NotPassableForEnemy,
    NotPassableForPlayer,
) {
    (
        Sprite {
            image,
            custom_size: Some(Vec2::splat(TILE_SIZE)),
            ..Default::default()
        },
        Transform::from_translation(position),
        Explodable,
        NotPassableForEnemy,
        NotPassableForPlayer,
    )
}

pub fn spawn_walls(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    let num_walls = 20;
    for x in 0..num_walls {
        let image_wall = asset_server.load("textures/wall.png");
        commands.spawn((
            Sprite {
                image: image_wall.clone(),
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                ..Default::default()
            },
            Transform::from_translation(Vec3::new(
                TILE_SIZE * x as f32 + WINDOW_WIDTH / 2.0,
                WINDOW_HEIGHT / 2.0,
                0.0,
            )),
            Explodable,
            NotPassableForEnemy,
            NotPassableForPlayer,
        ));
    }
}

pub fn spawn_wall_rectangles(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single();
    let image_wall = asset_server.load("textures/wall.png");
    let half_window_width = WINDOW_WIDTH / 2.0;
    let half_window_height = WINDOW_HEIGHT / 2.0;

    let rectangles = [(5.0, 5.0, 10.0, 6.0), (15.0, 10.0, 8.0, 4.0)];

    for &(start_x, start_y, width, height) in &rectangles {
        let tile_positions = (0..width as usize)
            .flat_map(|x| (0..height as usize).map(move |y| (x, y)))
            .filter(|&(x, y)| {
                x == 0 || x == (width as usize - 1) || y == 0 || y == (height as usize - 1)
            });

        for (x, y) in tile_positions {
            let world_x = start_x * TILE_SIZE + x as f32 * TILE_SIZE - half_window_width;
            let world_y = start_y * TILE_SIZE + y as f32 * TILE_SIZE - half_window_height;

            commands.spawn(create_wall_sprite(
                image_wall.clone(),
                Vec3::new(world_x, world_y, 1.0),
            ));
        }
    }
}
