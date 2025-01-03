use bevy::prelude::*;
use crate::*;
use crate::enemy::components::Enemy;
use player::resources::*;
use crate::moveable_elements::components::MovableElement;

pub const PLAYER_SPEED: f32 = 1.8;
pub const TRESHOLD: f32 = 1.0;


impl Default for PushCooldownTimer {
    fn default() -> Self {
        PushCooldownTimer(Timer::from_seconds(0.25, TimerMode::Repeating))
    }
}
pub fn setup_colision(mut commands: Commands) {
    commands.insert_resource(ColisionDebounce {
        timer: Timer::from_seconds(10.0, TimerMode::Once),
    });
}

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {

    let player_image = asset_server.load("textures/kretes.png");
    commands.spawn((
        Sprite {
            image: player_image,
            custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
            ..Default::default()
        },
        Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)), 
        GlobalTransform::default(),
        Player,
        Explodable,
        NotPassableForEnemy
    )
    );
}

pub fn player_dig_dirt(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    dirt_query: Query<(Entity, &Transform), With<Dirt>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        // Znajdź i usuń wszystkie obiekty Dirt na pozycji gracza
        dirt_query
            .iter()
            .filter(|(_, dirt_transform)| {
                dirt_transform.translation.truncate() == player_transform.translation.truncate()
            })
            .for_each(|(dirt_entity, _)| {
                commands.entity(dirt_entity).despawn();
            });
    }
}

fn create_planted_bomb_sprite() -> Sprite {
    Sprite {
        custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
        ..Default::default()
    }
}
fn is_within_hit_range(player_transform: &Transform, enemy_transform: &Transform) -> bool {
    (enemy_transform.translation.x - player_transform.translation.x).abs() < TILE_SIZE
        && (enemy_transform.translation.y - player_transform.translation.y).abs() < TILE_SIZE
}

fn spawn_planted_bomb(commands: &mut Commands, enemy_transform: &Transform, asset_server: &Res<AssetServer>) {
    commands.spawn((
        create_planted_bomb_sprite(),
        Transform::from_translation(Vec3::new(
            enemy_transform.translation.x,
            enemy_transform.translation.y,
            0.0,
        )),
        PlantedBomb,
    ))
    .insert(Lifetime {
        timer: Timer::from_seconds(0.0, TimerMode::Once),
    });

    commands.spawn(AudioPlayer::new(
        asset_server.load("audio/exploded_oneself.ogg"),
    ));
}

pub fn player_hit_enemy(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>,
    mut colision_debounce: ResMut<ColisionDebounce>,
    time: Res<Time>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        colision_debounce.timer.tick(time.delta());

        if colision_debounce.timer.finished() {
            for enemy_transform in enemy_query.iter() {
                if is_within_hit_range(player_transform, enemy_transform) {
                    spawn_planted_bomb(&mut commands, enemy_transform, &asset_server);
                    colision_debounce.timer.reset();
                }
            }
        }
    }
}

pub fn player_collect_bomb(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    bomb_query: Query<(Entity, &Transform), With<Bomb>>,
    mut bomb_count: ResMut<BombCount>,
) {
    // Sprawdzamy, czy gracz jest obecny w scenie
    if let Ok(player_transform) = player_query.get_single() {
        
        // Iterujemy przez wszystkie bomby
        for (bomb_entity, bomb_transform) in bomb_query.iter() {

            // Sprawdzamy, czy gracz znajduje się w pobliżu bomby
            if is_within_pickup_range(player_transform, bomb_transform) {
                // Zwiększamy liczbę bomb i usuwamy bombę z gry
                bomb_count.value += 1;
                commands.entity(bomb_entity).despawn();
            }
        }
    }
}

/// Funkcja pomocnicza sprawdzająca, czy gracz jest wystarczająco blisko bomby
fn is_within_pickup_range(player_transform: &Transform, bomb_transform: &Transform) -> bool {
    let player_pos = Vec2::new(player_transform.translation.x, player_transform.translation.y);
    let bomb_pos = Vec2::new(bomb_transform.translation.x, bomb_transform.translation.y);
    player_pos.distance(bomb_pos) < TILE_SIZE
}

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, (With<Player>, Without<NotPassableForPlayer>)>,
    not_walkable: Query<&Transform, With<NotPassableForPlayer>>,
    time: Res<Time>,
    mut move_cooldown: ResMut<PlayerMoveCooldown>,
) {
    // Jeśli gracz nie jest dostępny, nie wykonuj nic
    if let Ok(mut transform) = player_query.get_single_mut() {
        // Sprawdzenie, czy upłynęło wystarczająco dużo czasu od ostatniego ruchu
        if time.elapsed() - move_cooldown.last_move_time < Duration::from_secs_f32(0.2) {
            return;
        }

        // Wektor ruchu, który jest sumą kierunków w zależności od wciśniętych klawiszy
        let direction = get_direction_from_input(&keyboard_input);

        // Jeśli nie ma żadnego kierunku, nie wykonuj ruchu
        if direction == Vec3::ZERO {
            return;
        }

        let new_position = transform.translation + direction * TILE_SIZE;

        // Jeśli gracz nie może się poruszyć, sprawdź kolizję
        if is_position_blocked(new_position, &not_walkable) {
            return;
        }

        // Aktualizujemy pozycję gracza i zapisujemy czas ostatniego ruchu
        transform.translation = new_position;
        move_cooldown.last_move_time = time.elapsed();
    }
}

/// Funkcja zwraca wektor kierunku na podstawie wciśniętych klawiszy
fn get_direction_from_input(keyboard_input: &Res<ButtonInput<KeyCode>>) -> Vec3 {
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

    direction
}

/// Funkcja sprawdza, czy nowa pozycja gracza koliduje z jakąkolwiek przeszkodą
fn is_position_blocked(new_position: Vec3, not_walkable: &Query<&Transform, With<NotPassableForPlayer>>) -> bool {
    not_walkable.iter().any(|obstacle| {
        (new_position.x - obstacle.translation.x).abs() < TILE_SIZE
            && (new_position.y - obstacle.translation.y).abs() < TILE_SIZE
    })
}


pub fn player_push_system(
    mut queries: ParamSet<(
        Query<&Transform, With<Player>>, // Gracz
        Query<&mut Transform, With<MovableElement>>, // Elementy, które mogą być przesuwane
        Query<&Transform, With<NotPassableForEnemy>>, // Przeszkody, które blokują ruch
    )>,
    input: Res<ButtonInput<KeyCode>>, // Wejście z klawiatury
    mut cooldown_timer: ResMut<PushCooldownTimer>, // Timer do opóźnienia przepychania
    time: Res<Time>, // Czas, który będzie potrzebny do zaktualizowania timera
) {

    // Sprawdzamy, czy timer upłynął
    cooldown_timer.0.tick(time.delta());
    if !cooldown_timer.0.finished() {
        return; // Jeśli nie, nie wykonuj przepychania
    }

    // znajduje gracza
    if let Ok(player_transform) = queries.p0().get_single() {

        // pobieram pozycje gracza
        let push_direction = get_push_direction(&input); // Określ kierunek ruchu gracza
        let player_position = player_transform.translation + TILE_SIZE * push_direction;

        if push_direction != Vec3::ZERO {
            // Pobieramy przeszkody tylko raz
            let obstacles: Vec<Vec3> = queries.p2().iter().map(|obstacle| obstacle.translation).collect();

            // Iteruj przez obiekty, które można przesuwać
            for mut movable_transform in queries.p1().iter_mut() {
                if is_colliding_with_player(&player_position, &movable_transform.translation) {
                    println!("Kolizja z obiektem typu MovableElement");

                    let new_position = movable_transform.translation + TILE_SIZE * push_direction;

                    // Sprawdzamy, czy przesunięcie nie koliduje z przeszkodami
                    if !is_colliding_with_obstacles(&new_position, &obstacles) {
                        movable_transform.translation = new_position; // Przemieszczamy obiekt
                    } else {
                        println!("Nie można przesunąć: kolizja z obiektem NotPassableForEnemy");
                    }

                    break; // Jednocześnie można przesunąć tylko jeden obiekt
                }
            }
        }
    }
}

/// Funkcja zwraca wektor kierunku na podstawie wciśniętych klawiszy
fn get_push_direction(input: &Res<ButtonInput<KeyCode>>) -> Vec3 {
    if input.pressed(KeyCode::KeyW) {
        Vec3::new(0.0, 1.0, 0.0)
    } else if input.pressed(KeyCode::KeyS) {
        Vec3::new(0.0, -1.0, 0.0)
    } else if input.pressed(KeyCode::KeyA) {
        Vec3::new(-1.0, 0.0, 0.0)
    } else if input.pressed(KeyCode::KeyD) {
        Vec3::new(1.0, 0.0, 0.0)
    } else {
        Vec3::ZERO
    }
}

/// Funkcja sprawdzająca, czy pozycja obiektu koliduje z pozycją gracza
fn is_colliding_with_player(player_position: &Vec3, movable_position: &Vec3) -> bool {
    (movable_position.x - player_position.x).abs() + TRESHOLD < TILE_SIZE
        && (movable_position.y - player_position.y).abs() + TRESHOLD < TILE_SIZE
}

/// Funkcja sprawdzająca, czy nowa pozycja koliduje z jakąkolwiek przeszkodą
fn is_colliding_with_obstacles(new_position: &Vec3, obstacles: &[Vec3]) -> bool {
    obstacles.iter().any(|&obstacle| (obstacle - *new_position).length() < TILE_SIZE)
}
