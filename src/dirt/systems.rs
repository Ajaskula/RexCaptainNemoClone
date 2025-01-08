use crate::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn spawn_dirt(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    let image_bomb = asset_server.load("textures/dirt.png");

    // Dolne pozycje filarów
    let left_tower_base_x = 23.0 * TILE_SIZE;
    let right_tower_base_x = 25.0 * TILE_SIZE;
    let tower_base_y = -17.0 * TILE_SIZE;

    let tower_height = 5;

    // Tworzenie wieży po lewej stronie
    for i in 0..tower_height {
        commands.spawn((
            Dirt {},
            Sprite {
                image: image_bomb.clone(),
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                ..Default::default()
            },
            Transform::from_translation(Vec3::new(
                left_tower_base_x,
                tower_base_y + i as f32 * TILE_SIZE,
                0.0,
            )),
            Explodable {},
            NotPassableForEnemy,
        ));
    }

    // Tworzenie wieży po prawej stronie
    for i in 0..tower_height {
        commands.spawn((
            Dirt {},
            Sprite {
                image: image_bomb.clone(),
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                ..Default::default()
            },
            Transform::from_translation(Vec3::new(
                right_tower_base_x,
                tower_base_y + i as f32 * TILE_SIZE,
                0.0,
            )),
            Explodable {},
            NotPassableForEnemy,
        ));
    }

    // Tworzenie bloku na szczycie pomiędzy wieżami
    let middle_block_y = tower_base_y + (tower_height as f32 - 1.0) * TILE_SIZE; // Wysokość bloku pomiędzy
    let middle_block_x = (left_tower_base_x + right_tower_base_x) / 2.0; // Środkowa pozycja X

    commands.spawn((
        Dirt {},
        Sprite {
            image: image_bomb.clone(),
            custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
            ..Default::default()
        },
        Transform::from_translation(Vec3::new(middle_block_x, middle_block_y, 0.0)),
        Explodable {},
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

    let rectangles = vec![(5.0, 5.0, 8.0, 6.0), (15.0, 10.0, 10.0, 5.0)];

    for (start_x, start_y, width, height) in rectangles {
        for x in 0..(width as usize) {
            for y in 0..(height as usize) {
                let world_x = start_x * tile_size + x as f32 * tile_size - window.width();
                let world_y = start_y * tile_size + y as f32 * tile_size - window.height();

                commands.spawn((
                    Dirt {},
                    Sprite {
                        image: image_dirt.clone(),
                        custom_size: Some(Vec2::new(tile_size, tile_size)),
                        ..Default::default()
                    },
                    Transform::from_translation(Vec3::new(
                        world_x + 20.0 * TILE_SIZE,
                        world_y + 20.0 * TILE_SIZE,
                        0.0,
                    )),
                    Explodable {},
                    NotPassableForEnemy,
                ));
            }
        }
    }
}
