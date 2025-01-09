use crate::enemy::components::Enemy;
use crate::moveable_elements::components::MovableElement;
use crate::player::config::{PLAYER_STARTING_TILE_POSITION, THRESHOLD};
use crate::*;
use bevy::prelude::*;
use player::resources::*;

impl Default for PushCooldownTimer {
    fn default() -> Self {
        PushCooldownTimer(Timer::from_seconds(0.23, TimerMode::Repeating))
    }
}
pub fn setup_collision(mut commands: Commands) {
    commands.insert_resource(CollisionDebounce {
        timer: Timer::from_seconds(10.0, TimerMode::Once),
    });
}

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let player_image = asset_server.load("textures/kretes.png");
    commands.spawn((
        Player,
        Sprite {
            image: player_image,
            custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
            ..Default::default()
        },
        Transform::from_translation(Vec3::new(
            PLAYER_STARTING_TILE_POSITION.0 * TILE_SIZE,
            PLAYER_STARTING_TILE_POSITION.1 * TILE_SIZE,
            3.0,
        )),
        GlobalTransform::default(),
        Explodable,
        NotPassableForEnemy,
    ));
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

fn spawn_planted_bomb(
    commands: &mut Commands,
    enemy_transform: &Transform,
    asset_server: &Res<AssetServer>,
) {
    commands.spawn((
        PlantedBomb,
        create_planted_bomb_sprite(),
        Transform::from_translation(Vec3::new(
            enemy_transform.translation.x,
            enemy_transform.translation.y,
            0.0,
        )),
        Lifetime {
            timer: Timer::from_seconds(0.0, TimerMode::Once),
        },
    ));
    commands.spawn(AudioPlayer::new(
        asset_server.load("audio/exploded_oneself.ogg"),
    ));
}

pub fn player_hit_enemy(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>,
    mut collision_debounce: ResMut<CollisionDebounce>,
    time: Res<Time>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        collision_debounce.timer.tick(time.delta());
        if collision_debounce.timer.finished() {
            for enemy_transform in enemy_query.iter() {
                if is_within_hit_range(player_transform, enemy_transform) {
                    spawn_planted_bomb(&mut commands, enemy_transform, &asset_server);
                    collision_debounce.timer.reset();
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
    if let Ok(player_transform) = player_query.get_single() {
        for (bomb_entity, bomb_transform) in bomb_query.iter() {
            if is_within_pickup_range(player_transform, bomb_transform) {
                bomb_count.value += 1;
                commands.entity(bomb_entity).despawn();
            }
        }
    }
}

/// Funkcja pomocnicza sprawdzająca, czy gracz jest wystarczająco blisko bomby
fn is_within_pickup_range(player_transform: &Transform, bomb_transform: &Transform) -> bool {
    let player_pos = Vec2::new(
        player_transform.translation.x,
        player_transform.translation.y,
    );
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
    if let Ok(mut transform) = player_query.get_single_mut() {
        if time.elapsed() - move_cooldown.last_move_time < Duration::from_secs_f32(0.2) {
            return;
        }

        let direction = get_direction_from_input(&keyboard_input);
        if direction == Vec3::ZERO {
            return;
        }

        let new_position = transform.translation + direction * TILE_SIZE;
        if is_position_blocked(new_position, &not_walkable) {
            return;
        }

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
fn is_position_blocked(
    new_position: Vec3,
    not_walkable: &Query<&Transform, With<NotPassableForPlayer>>,
) -> bool {
    not_walkable.iter().any(|obstacle| {
        (new_position.x - obstacle.translation.x).abs() + THRESHOLD < TILE_SIZE
            && (new_position.y - obstacle.translation.y).abs() + THRESHOLD < TILE_SIZE
    })
}

pub fn player_push_system(
    mut queries: ParamSet<(
        Query<&Transform, With<Player>>,
        Query<&mut Transform, With<MovableElement>>,
        Query<&Transform, With<NotPassableForEnemy>>,
    )>,
    input: Res<ButtonInput<KeyCode>>,
    mut cooldown_timer: ResMut<PushCooldownTimer>,
    time: Res<Time>,
) {
    cooldown_timer.0.tick(time.delta());
    if !cooldown_timer.0.finished() {
        return;
    }

    if let Ok(player_transform) = queries.p0().get_single() {
        let push_direction = get_push_direction(&input);
        let player_position = player_transform.translation + TILE_SIZE * push_direction;

        if push_direction != Vec3::ZERO {
            let obstacles: Vec<Vec3> = queries
                .p2()
                .iter()
                .map(|obstacle| obstacle.translation)
                .collect();

            for mut movable_transform in queries.p1().iter_mut() {
                if is_colliding_with_player(&player_position, &movable_transform.translation) {
                    println!("Kolizja z obiektem typu MovableElement");

                    let new_position = movable_transform.translation + TILE_SIZE * push_direction;

                    if !is_colliding_with_obstacles(&new_position, &obstacles) {
                        movable_transform.translation = new_position;
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
    if input.pressed(KeyCode::KeyA) {
        Vec3::new(-1.0, 0.0, 0.0)
    } else if input.pressed(KeyCode::KeyD) {
        Vec3::new(1.0, 0.0, 0.0)
    } else {
        Vec3::ZERO
    }
}

/// Funkcja sprawdzająca, czy pozycja obiektu koliduje z pozycją gracza
fn is_colliding_with_player(player_position: &Vec3, movable_position: &Vec3) -> bool {
    let is_y_close = (movable_position.y - player_position.y).abs() < THRESHOLD;
    let is_x_exact = movable_position.x == player_position.x;
    is_y_close && is_x_exact
}

/// Funkcja sprawdzająca, czy nowa pozycja koliduje z jakąkolwiek przeszkodą
fn is_colliding_with_obstacles(new_position: &Vec3, obstacles: &[Vec3]) -> bool {
    obstacles
        .iter()
        .any(|&obstacle| (obstacle - *new_position).length() < TILE_SIZE) // TODO: it can still collide even when >=
}
