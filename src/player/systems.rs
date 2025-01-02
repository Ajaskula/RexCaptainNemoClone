use bevy::prelude::*;
use crate::*;
use crate::enemy::components::Enemy;
use player::resources::*;
use crate::moveable_elements::components::MovableElement;

pub const PLAYER_SPEED: f32 = 1.8;
pub const TRESHOLD: f32 = 1.0;




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
    keyboard_input: Res<ButtonInput<KeyCode>>, // Wejście z klawiatury
    mut player_query: Query<&mut Transform, (With<Player>, Without<NotPassableForPlayer>)>, // Zapytanie o gracza
    not_walkable: Query<&Transform, With<NotPassableForPlayer>>, // Zapytanie o przeszkody, które nie są do przejścia
    time: Res<Time>, // Czas (dla zarządzania szybkością poruszania się)
    mut move_cooldown: ResMut<PlayerMoveCooldown>, // Zmienna cooldownu ruchu
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        // Sprawdź czas od ostatniego ruchu
        if time.elapsed() - move_cooldown.last_move_time < Duration::from_secs_f32(0.2) {
            return; // Za wcześnie, aby poruszać się ponownie
        }

        // Określenie kierunku w zależności od naciśniętych klawiszy
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

        if direction == Vec3::ZERO {
            return; // Nie ma wejścia, nie poruszaj się
        }

        let new_position = transform.translation + direction * TILE_SIZE;

        // Sprawdzamy, czy gracz koliduje z jakąś przeszkodą
        for obstacle in not_walkable.iter() {
            if (new_position.x - obstacle.translation.x).abs() < TILE_SIZE
                && (new_position.y - obstacle.translation.y).abs() < TILE_SIZE
            {
                return; // Kolizja, nie można się poruszyć
            }
        }

        // Jeśli nie ma kolizji, aktualizujemy pozycję gracza
        transform.translation = new_position;
        move_cooldown.last_move_time = time.elapsed(); // Zapisz czas ruchu
    }
}


pub fn player_push_system(
    mut queries: ParamSet<(
        Query<&Transform, With<Player>>,
        Query<&mut Transform, With<MovableElement>>,
        Query<&Transform, With<NotPassableForEnemy>>,
    )>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if let Ok(player_transform) = queries.p0().get_single() {
        let player_position = player_transform.translation; // Skopiuj pozycję gracza
        let mut push_direction = Vec3::ZERO;

        // Wykrywaj kierunek ruchu gracza
        if input.pressed(KeyCode::KeyW) {
            push_direction = Vec3::new(0.0, 1.0, 0.0);
        } else if input.pressed(KeyCode::KeyS) {
            push_direction = Vec3::new(0.0, -1.0, 0.0);
        } else if input.pressed(KeyCode::KeyA) {
            push_direction = Vec3::new(-1.0, 0.0, 0.0);
        } else if input.pressed(KeyCode::KeyD) {
            push_direction = Vec3::new(1.0, 0.0, 0.0);
        }

        if push_direction != Vec3::ZERO {
            let target_position = player_position + TILE_SIZE * push_direction;

            // Pobierz wszystkie pozycje przeszkód do wektora
            let obstacles: Vec<Vec3> = queries
                .p2()
                .iter()
                .map(|obstacle_transform| obstacle_transform.translation)
                .collect();

            // Iteruj przez obiekty `Plague`
            for mut plague_transform in queries.p1().iter_mut() {
                // Sprawdź, czy jest kolizja między graczem a obiektem typu `Plague`
                if (plague_transform.translation.x - player_position.x).abs() + TRESHOLD < TILE_SIZE
                && (plague_transform.translation.y - player_position.y).abs() + TRESHOLD < TILE_SIZE
                 {
                    println!("Kolizja z obiektem typu Plague");

                    let new_position = plague_transform.translation + TILE_SIZE * push_direction;

                    // Sprawdź, czy nowa pozycja koliduje z przeszkodami
                    let collision = obstacles.iter().any(|&obstacle| {
                        (obstacle - new_position).length() < TILE_SIZE
                    });

                    if !collision {
                        plague_transform.translation = new_position;
                    } else {
                        println!("Nie można przesunąć: kolizja z obiektem NotPassableForEnemy");
                    }

                    break;
                }
            }
        }
    }
}
