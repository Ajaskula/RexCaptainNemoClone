use bevy::prelude::*;
use crate::*;
use crate::enemy::components::Enemy;
use player::resources::*;
use crate::plague::components::Plague;

pub const PLAYER_SPEED: f32 = 0.2;


pub fn setup_colision(mut commands: Commands) {
    commands.insert_resource(ColisionDebounce {
        timer: Timer::from_seconds(1.0, TimerMode::Once),
    });
}

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {

    // let window = window_query.get_single().unwrap();
    let player_image = asset_server.load("textures/kretes.png");
    commands.spawn((
        Sprite {
            image: player_image,
            custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
            ..Default::default()
        },
        Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)), 
        GlobalTransform::default(),
        Player{},
        Explodable{},
        NotPassableForEnemy
    )
    );
}

pub fn player_dig_dirt(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    dirt_query: Query<(Entity, &Transform), With<Dirt>>
){
    if let Ok(transform) = player_query.get_single() {

        for (dirt_entity, dirt_transform) in dirt_query.iter() {

            if (dirt_transform.translation.x - transform.translation.x).abs() < TILE_SIZE && (dirt_transform.translation.y - transform.translation.y).abs() < TILE_SIZE {
                commands.entity(dirt_entity).despawn();
                println!("Despawnuje dirt")
            } 
        }
    }
}

pub fn player_hit_enemy(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>,
    mut colision_debounce: ResMut<ColisionDebounce>,
    time: Res<Time>,
){

    if let Ok(transform) = player_query.get_single() {
        for enemy_transform in enemy_query.iter() {

            colision_debounce.timer.tick(time.delta());
            if colision_debounce.timer.finished() && (enemy_transform.translation.x - transform.translation.x).abs() < TILE_SIZE && (enemy_transform.translation.y - transform.translation.y).abs() < TILE_SIZE {
                //commands.entity(enemy_entity).despawn();
                
                // spawnuje enemy
                commands// spawnuje bombe w podanej lokalizacji
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
                ))// dokładam do tego sprite lifetime z timerem
                .insert(Lifetime {
                    timer: Timer::from_seconds(0.0, TimerMode::Once),
                });
                commands.spawn(
                    AudioPlayer::new(
                        asset_server.load("audio/exploded_oneself.ogg"),
                    )
                );

                colision_debounce.timer.reset();
            }
        }
    }
    
}

pub fn player_collect_bomb(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    bomb_query: Query<(Entity, &Transform), With<Bomb>>,
    mut bomb_count: ResMut<BombCount>
    // audio: Res<Audio>
){
    if let Ok(transform) = player_query.get_single() {

        for (bomb_entity, bomb_transform) in bomb_query.iter() {

            if (bomb_transform.translation.x - transform.translation.x).abs() < TILE_SIZE && (bomb_transform.translation.y - transform.translation.y).abs() < TILE_SIZE {
                // let sound_effect = asset_server.load("audio/.");
                bomb_count.value += 1;
                commands.entity(bomb_entity).despawn();
                println!("Zbieram bombę, liczba bomb = {}", bomb_count.value);
            } 
        }
    }
}


pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, (With<Player>, Without<NotPassableForPlayer>)>,
    not_walkable: Query<&Transform, With<NotPassableForPlayer>>,
    time: Res<Time>
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::KeyW) {
            direction = Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            direction = Vec3::new(0.0, -1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            direction = Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            direction = Vec3::new(1.0, 0.0, 0.0);
        }

        let mut collision = false;

        for obstacle in not_walkable.iter() {
            let new_x = (transform.translation.x + direction.x * PLAYER_SPEED);
            let new_y = (transform.translation.y + direction.y * PLAYER_SPEED);

            if (new_x - obstacle.translation.x).abs() < TILE_SIZE 
                && (new_y - obstacle.translation.y).abs() < TILE_SIZE 
            {
                collision = true;
                break; // Nie musimy dalej sprawdzać
            }
        }

        if !collision {
            transform.translation += direction * PLAYER_SPEED * TILE_SIZE;
            // transform.translation.x = (transform.translation.x / TILE_SIZE).round() * TILE_SIZE;
            // transform.translation.y = (transform.translation.y / TILE_SIZE).round() * TILE_SIZE;
        }
    }
}


pub fn player_push_system(
    mut queries: ParamSet<(
        Query<&Transform, With<Player>>,
        Query<&mut Transform, With<Plague>>,
        Query<&Transform, With<NotPassableForEnemy>>,
    )>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if let Ok(player_transform) = queries.p0().get_single() {
        let mut push_direction = Vec3::ZERO;

        // Wykrywaj kierunek ruchu gracza
        if input.just_pressed(KeyCode::KeyW) {
            push_direction = Vec3::new(0.0, 1.0, 0.0);
        } else if input.just_pressed(KeyCode::KeyS) {
            push_direction = Vec3::new(0.0, -1.0, 0.0);
        } else if input.just_pressed(KeyCode::KeyA) {
            push_direction = Vec3::new(-1.0, 0.0, 0.0);
        } else if input.just_pressed(KeyCode::KeyD) {
            push_direction = Vec3::new(1.0, 0.0, 0.0);
        }

        if push_direction != Vec3::ZERO {
            println!("wchodzę w push różny od 0");
            let target_position = player_transform.translation + push_direction;

            // Przechowaj przeszkody w wektorze, aby uniknąć jednoczesnego dostępu do `queries.p2()`
            let obstacles: Vec<Vec3> = queries
                .p2()
                .iter()
                .map(|obstacle| obstacle.translation)
                .collect();
            
            // przechodzę przez całą tablicę not passable for enemy
            for mut plague_transform in queries.p1().iter_mut() {
                if (plague_transform.translation - target_position).length() < TILE_SIZE {
                    let new_position = plague_transform.translation + push_direction;

                    let collision = obstacles.iter().any(|&obstacle| {
                        (obstacle - new_position).length() < TILE_SIZE
                    });

                    if !collision {
                        plague_transform.translation = new_position;
                    }
                    break;
                }
            }
        }
    }
}
