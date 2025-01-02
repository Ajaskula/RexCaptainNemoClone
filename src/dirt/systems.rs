use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::*;

pub fn spawn_dirt(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>
){

    let window = window_query.get_single().unwrap();
    let num_walls = 15;
    // let mut rng = rand::thread_rng();
    let image_bomb = asset_server.load("textures/dirt.png");
    // for x in 0..num_walls {
    //     // let x = rng.gen_range(0..=(window.width() / TILE_SIZE) as usize) * TILE_SIZE as usize;
    //     // let y = rng.gen_range(0..=(window.height() / TILE_SIZE) as usize) * TILE_SIZE as usize;
    //     commands.spawn((
    //         Sprite {
    //             image:image_bomb.clone(),
    //             custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
    //             ..Default::default()
    //         },
    //         Transform::from_translation(Vec3::new(24.0 * TILE_SIZE as f32, 4.0 * TILE_SIZE as f32, 0.0)),
    //         Dirt{},
    //         Explodable{}
    //     )
    // );
    // }
    commands.spawn((
        Sprite {
            image:image_bomb.clone(),
            custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
            ..Default::default()
        },
        Transform::from_translation(Vec3::new(25.0 * TILE_SIZE as f32, -16.0 * TILE_SIZE as f32, 0.0)),
        Dirt{},
        Explodable{},
        NotPassableForEnemy,
    ));
    commands.spawn((
        Sprite {
            image:image_bomb.clone(),
            custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
            ..Default::default()
        },
        Transform::from_translation(Vec3::new(25.0 * TILE_SIZE as f32, -17.0 * TILE_SIZE as f32, 0.0)),
        Dirt{},
        Explodable{},
        NotPassableForEnemy,
    ));
    commands.spawn((
        Sprite {
            image:image_bomb.clone(),
            custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
            ..Default::default()
        },
        Transform::from_translation(Vec3::new(23.0 * TILE_SIZE as f32, -16.0 * TILE_SIZE as f32, 0.0)),
        Dirt{},
        Explodable{},
        NotPassableForEnemy,
    ));
    commands.spawn((
        Sprite {
            image:image_bomb.clone(),
            custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
            ..Default::default()
        },
        Transform::from_translation(Vec3::new(23.0 * TILE_SIZE as f32, -17.0 * TILE_SIZE as f32, 0.0)),
        Dirt{},
        Explodable{},
        NotPassableForEnemy,
    ));
}

pub fn spawn_full_dirt_rectangles(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    let image_dirt = asset_server.load("textures/dirt.png");
    let tile_size = TILE_SIZE;

    // Lista prostokątów do wygenerowania: (start_x, start_y, width, height)
    let rectangles = vec![
        (5.0, 5.0, 8.0, 6.0),
        (15.0, 10.0, 10.0, 5.0),
    ];

    for (start_x, start_y, width, height) in rectangles {
        for x in 0..(width as usize) {
            for y in 0..(height as usize) {
                // Oblicz współrzędne świata dla każdego kafelka
                let world_x = start_x * tile_size + x as f32 * tile_size - window.width();
                let world_y = start_y * tile_size + y as f32 * tile_size - window.height();

                // Tworzenie kafelka
                commands.spawn((
                    Sprite {
                        image: image_dirt.clone(),
                        custom_size: Some(Vec2::new(tile_size, tile_size)),
                        ..Default::default()
                    },
                    Transform::from_translation(Vec3::new(world_x + 20.0*TILE_SIZE, world_y+20.0*TILE_SIZE, 0.0)),
                    Dirt {},          // Oznaczamy jako Dirt
                    Explodable {},    // Może być eksplodowane
                    NotPassableForEnemy,
                ));
            }
        }
    }
}


