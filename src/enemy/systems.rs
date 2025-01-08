use crate::*;
use bevy::prelude::*;
use rand::Rng;

use crate::enemy::components::Enemy;
use crate::moveable_elements::components::*;
use crate::player::resources::*;

pub const ENEMY_SPEED: f32 = 50.0;

const ENEMY_DIRECTIONS_ARRAY: [Vec2; 4] = [
    Vec2::new(1.0, 0.0),
    Vec2::new(0.0, 1.0),
    Vec2::new(-1.0, 0.0),
    Vec2::new(0.0, -1.0),
];

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

        // Oblicz współrzędne w przestrzeni świata
        let world_x = rand_x * tile_size - window.width() / 2.0;
        let world_y = rand_y * tile_size - window.height() / 2.0;

        // Tworzymy wroga
        commands.spawn((
            Sprite {
                image: image_enemy.clone(),
                custom_size: Some(Vec2::new(tile_size, tile_size)),
                ..Default::default()
            },
            Transform::from_translation(Vec3::new(world_x, world_y, 0.0)),
            Enemy {
                num: 0,
                direction: Vec2::new(1.0, 0.0),
            },
            Explodable {}, // Dodajemy komponent Explodable, by wróg mógł wybuchnąć
        ));
    }
}

pub fn spawn_enemy(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Pozycje X wież z funkcji spawn_dirt
    let left_tower_x = 23.0 * TILE_SIZE as f32;
    let right_tower_x = 25.0 * TILE_SIZE as f32;
    let tower_base_y = -17.0 * TILE_SIZE as f32;

    // Obliczenie pozycji startowej dla Enemy
    let enemy_start_x = (left_tower_x + right_tower_x) / 2.0;
    let enemy_start_y = tower_base_y + TILE_SIZE; // Tuż nad podstawą wież

    // Załadowanie tekstury Enemy
    let enemy_texture = asset_server.load("textures/mumionek.png");

    // Tworzenie Enemy
    commands.spawn((
        Sprite {
            image: enemy_texture.clone(),
            custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
            ..Default::default()
        },
        Transform::from_translation(Vec3::new(enemy_start_x, enemy_start_y, 0.0)),
        Enemy {
            num: 0,
            direction: Vec2::new(0.0, -1.0), // Początkowy kierunek Enemy (np. w dół)
        },
        NotPassableForPlayer,
        Explodable {},
        Explosive {}, // Enemy powinno blokować gracza
    ));
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
            let mut rng = rand::thread_rng(); // Tworzenie generatora liczb losowych
            let rand_num = rng.gen_range(0..=3); // Generowanie liczby z zakresu 0..=3 (włącznie)

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
    mut collision_debounce: ResMut<ColisionDebounce>,
    time: Res<Time>,
) {
    // Przechodzimy przez wszystkie spadające elementy
    for falling_transform in falling_query.iter() {
        for (enemy_transform, enemy_entity) in enemy_query.iter() {
            // Sprawdzamy, czy spadający element jest nad przeciwnikiem
            let is_above = (falling_transform.translation.x - enemy_transform.translation.x).abs()
                < TILE_SIZE
                && (falling_transform.translation.y > enemy_transform.translation.y)
                && (falling_transform.translation.y - enemy_transform.translation.y).abs()
                    < TILE_SIZE;

            collision_debounce.timer.tick(time.delta());
            if collision_debounce.timer.finished() && is_above {
                // Zniszcz przeciwnika
                commands // spawnuje bombe w podanej lokalizacji
                    .spawn((
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
                        PlantedBomb {},
                    )) // dokładam do tego sprite lifetime z timerem
                    .insert(Lifetime {
                        timer: Timer::from_seconds(0.0, TimerMode::Once),
                    });
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
