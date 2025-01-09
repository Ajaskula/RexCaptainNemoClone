use crate::*;
use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::enemy::components::Enemy;
use crate::enemy::config::{ENEMY_DIRECTIONS_ARRAY, ENEMY_SPEED};
use crate::moveable_elements::components::*;
use crate::player::config::{INITIAL_SAFE_ZONE_TILE_DISTANCE, PLAYER_STARTING_TILE_POSITION};
use crate::player::resources::*;

pub fn spawn_enemies(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    let image_enemy = asset_server.load("textures/mumionek.png");
    let tile_size = TILE_SIZE;

    // Lista prostokątów, w których będą spawnować się wrogowie
    let rectangles = vec![
        (5.0, 5.0, 8.0, 6.0), // (start_x, start_y, width, height)
        (15.0, 10.0, 10.0, 5.0),
    ];

    // Wygenerowanie wroga w każdym prostokącie
    for (start_x, start_y, width, height) in rectangles {
        // Losowa pozycja w obrębie prostokąta (tylko jeden wróg na prostokąt)
        let rand_x = start_x + rand::random::<f32>() * width;
        let rand_y = start_y + rand::random::<f32>() * height;

        let world_x = rand_x * tile_size - WINDOW_WIDTH / 2.0;
        let world_y = rand_y * tile_size - WINDOW_HEIGHT / 2.0;

        commands.spawn((
            Enemy {
                num: 0,
                direction: Vec2::new(1.0, 0.0),
            },
            Sprite {
                image: image_enemy.clone(),
                custom_size: Some(Vec2::new(tile_size, tile_size)),
                ..Default::default()
            },
            Transform::from_translation(Vec3::new(world_x, world_y, 2.0)),
            Explodable {},
        ));
    }
}

pub fn spawn_enemy(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Pozycje X wież z funkcji spawn_dirt
    let left_tower_x = 23.0 * TILE_SIZE;
    let right_tower_x = 25.0 * TILE_SIZE;
    let tower_base_y = -17.0 * TILE_SIZE;

    let enemy_start_x = (left_tower_x + right_tower_x) / 2.0;
    let enemy_start_y = tower_base_y + TILE_SIZE; // Tuż nad podstawą wież

    let enemy_texture = asset_server.load("textures/mumionek.png");

    let mut rng = thread_rng();
    for _ in 0..500 {
        let (mut x, mut y) = (
            rng.gen_range(-WINDOW_WIDTH_TILES + 1..WINDOW_WIDTH_TILES),
            rng.gen_range(-WINDOW_HEIGHT_TILES + 1..WINDOW_HEIGHT_TILES),
        );
        while Vec2::new(x as f32, y as f32).distance(Vec2::new(
            PLAYER_STARTING_TILE_POSITION.0,
            PLAYER_STARTING_TILE_POSITION.1,
        )) < INITIAL_SAFE_ZONE_TILE_DISTANCE as f32
        {
            (x, y) = (
                rng.gen_range(-WINDOW_WIDTH_TILES + 1..WINDOW_WIDTH_TILES),
                rng.gen_range(-WINDOW_HEIGHT_TILES + 1..WINDOW_HEIGHT_TILES),
            );
        }

        commands.spawn((
            Enemy {
                num: 0,
                direction: Vec2::new(0.0, -1.0),
            },
            Sprite {
                image: enemy_texture.clone(),
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                ..Default::default()
            },
            Transform::from_translation(Vec3::new(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE, 2.0)),
            NotPassableForPlayer,
            Explodable {},
            Explosive {}, // Enemy powinno blokować gracza
        ));
    }
}

pub fn enemy_movement(
    mut enemy_query: Query<(&mut Transform, &mut Enemy), Without<NotPassableForEnemy>>,
    time: Res<Time>,
    not_walkable: Query<&Transform, With<NotPassableForEnemy>>,
) {
    for (mut transform, mut enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        let mut collision = false;

        let time_delta = time.delta_secs();
        let new_position = transform.translation + direction * ENEMY_SPEED * time_delta;

        for obstacle in not_walkable.iter() {
            if (new_position.x - obstacle.translation.x).abs() < TILE_SIZE
                && (new_position.y - obstacle.translation.y).abs() < TILE_SIZE
            {
                collision = true;
                break;
            }
        }

        if !collision {
            transform.translation = new_position;
        } else {
            let mut rng = rand::thread_rng();
            let rand_num = rng.gen_range(0..=3);

            enemy.direction = ENEMY_DIRECTIONS_ARRAY[rand_num as usize];
        }
    }
}

//
pub fn enemy_hit_moveable_element(
    mut commands: Commands,
    falling_query: Query<&Transform, With<MovableElement>>,
    enemy_query: Query<(&Transform, Entity), With<Enemy>>,
    asset_server: Res<AssetServer>,
    mut collision_debounce: ResMut<CollisionDebounce>,
    time: Res<Time>,
) {
    for falling_transform in falling_query.iter() {
        for (enemy_transform, enemy_entity) in enemy_query.iter() {
            let is_above_enemy =
                (falling_transform.translation.x - enemy_transform.translation.x).abs() < TILE_SIZE
                    && (falling_transform.translation.y > enemy_transform.translation.y)
                    && (falling_transform.translation.y - enemy_transform.translation.y).abs()
                        < TILE_SIZE;

            collision_debounce.timer.tick(time.delta());
            if collision_debounce.timer.finished() && is_above_enemy {
                // Zniszcz przeciwnika
                commands.spawn((
                    PlantedBomb {},
                    Sprite {
                        // image: asset_server.load("textures/bomb.png").clone(),
                        custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                        ..Default::default()
                    },
                    Transform::from_translation(Vec3::new(
                        enemy_transform.translation.x,
                        enemy_transform.translation.y,
                        0.0,
                    )),
                    Lifetime {
                        timer: Timer::from_seconds(0.0, TimerMode::Once),
                    },
                ));
                // commands.spawn(
                //     AudioPlayer::new(
                //         asset_server.load("audio/exploded_oneself.ogg"),
                //     )
                // );

                collision_debounce.timer.reset();
            }
        }
    }
}
