use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::*;

pub fn spawn_walls(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>
){

    let window = window_query.get_single().unwrap();
    let num_walls = 20;
    for x in 0..num_walls {
        let image_wall = asset_server.load("textures/wall.png");
        commands.spawn((
            Sprite {
                image:image_wall.clone(),
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                ..Default::default()
            },
            Transform::from_translation(Vec3::new(TILE_SIZE*x as f32 + window.width() / 2.0, window.height() / 2.0, 0.0)),
            // SolidWall{},
            Explodable{},
            NotWalkable,
        )
    );
    }
}

pub fn spawn_wall_rectangles(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    let image_wall = asset_server.load("textures/wall.png");
    let tile_size = TILE_SIZE;

    // Parametry prostokąta
    let rectangles = vec![
        // Pozycja startowa (x, y), szerokość (w) i wysokość (h)
        (5.0, 5.0, 10.0, 6.0),
        (15.0, 10.0, 8.0, 4.0),
    ];

    for (start_x, start_y, width, height) in rectangles {
        for x in 0..(width as usize) {
            for y in 0..(height as usize) {
                // Generuj tylko krawędzie (górną, dolną, lewą i prawą)
                if x == 0 || x == (width as usize - 1) || y == 0 || y == (height as usize - 1) {
                    let world_x = start_x * tile_size + x as f32 * tile_size - window.width() / 2.0;
                    let world_y = start_y * tile_size + y as f32 * tile_size - window.height() / 2.0;

                    commands.spawn((
                        Sprite {
                            image: image_wall.clone(),
                            custom_size: Some(Vec2::new(tile_size, tile_size)),
                            ..Default::default()
                        },
                        Transform::from_translation(Vec3::new(world_x, world_y, 0.0)),
                        Explodable {},
                        NotWalkable,
                    ));
                }
            }
        }
    }
}

