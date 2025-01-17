use crate::moveable_elements::components::*;
use crate::moveable_elements::config::{FALLING_SPEED, THRESHOLD};
use crate::*;

fn get_movable_element(
    position: Vec3,
    image_path: &str,
    movable_element: MovableElement,
    asset_server: &Res<AssetServer>,
) -> (
    Sprite,
    Transform,
    MovableElement,
    Explodable,
    NotPassableForEnemy,
    NotPassableForPlayer,
) {
    let image = asset_server.load(image_path);

    let sprite = Sprite {
        image: image.clone(),
        custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
        ..Default::default()
    };

    let transform = Transform::from_translation(position);

    // Zwróć krotkę z wszystkimi komponentami do spawnu
    (
        sprite,
        transform,
        movable_element,
        Explodable,
        NotPassableForEnemy,
        NotPassableForPlayer,
    )
}

fn get_explosive_movable_element(
    position: Vec3,
    image_path: &str,
    movable_element: MovableElement,
    asset_server: &Res<AssetServer>,
) -> (
    Sprite,
    Transform,
    MovableElement,
    Explodable,
    NotPassableForEnemy,
    NotPassableForPlayer,
    Explosive,
) {
    let (sprite, transform, element, explodable, not_passable_enemy, not_passable_player) =
        get_movable_element(position, image_path, movable_element, asset_server);

    (
        sprite,
        transform,
        element,
        explodable,
        not_passable_enemy,
        not_passable_player,
        Explosive, // Dodajemy Explosive do krotki
    )
}

pub fn spawn_rocks(mut commands: Commands, asset_server: Res<AssetServer>, window: Query<&Window>) {
    let image_path = "textures/rock.png";

    let window_width = window.single().width();
    let window_height = window.single().height();

    let position = Vec3::new(-window_width + TILE_SIZE, window_height - TILE_SIZE, 0.0);

    commands.spawn(get_movable_element(
        position,
        image_path,
        MovableElement::Rock,
        &asset_server,
    ));
}

pub fn spawn_falling_bomb(mut commands: Commands, asset_server: Res<AssetServer>) {
    let position = Vec3::new(11.0 * TILE_SIZE, 3.0 * TILE_SIZE, 0.0);
    let image_path = "textures/bomb_falling.png";
    let movable_element = MovableElement::FallingBomb;
    commands.spawn(get_explosive_movable_element(
        position,
        image_path,
        movable_element,
        &asset_server,
    ));
}

pub fn moveable_elements_movement(
    mut plague_query: Query<(&mut Transform, &MovableElement)>,
    not_passable: Query<&Transform, (With<NotPassableForEnemy>, Without<MovableElement>)>, // Without to satisfy borrow checker
) {
    let plague_positions: Vec<Vec3> = plague_query
        .iter()
        .map(|(transform, _)| transform.translation)
        .collect();

    for (index, (mut transform, _)) in plague_query.iter_mut().enumerate() {
        let mut collision = false;
        let direction = Vec3::new(0.0, -1.0, 0.0);
        let new_position = transform.translation + direction * FALLING_SPEED;

        for obstacle in not_passable.iter() {
            if ((new_position.x - obstacle.translation.x).abs() < TILE_SIZE)
                && ((new_position.y - obstacle.translation.y).abs() <= TILE_SIZE)
            {
                collision = true;
                break;
            }
        }

        for (other_index, other_position) in plague_positions.iter().enumerate() {
            if index != other_index
                && (new_position.x - other_position.x).abs() < TILE_SIZE
                && (new_position.y - other_position.y).abs() <= TILE_SIZE
            {
                collision = true;
                break;
            }
        }

        if !collision {
            transform.translation = new_position;
        } else {
            let mut is_stacked = false;
            for (other_index, other_position) in plague_positions.iter().enumerate() {
                if index != other_index
                    && (transform.translation.x - other_position.x).abs() < TILE_SIZE
                    && (transform.translation.y - other_position.y).abs() <= TILE_SIZE + THRESHOLD
                    && transform.translation.y > other_position.y
                {
                    is_stacked = true;
                    break;
                }
            }

            if is_stacked {
                let left_position = transform.translation + Vec3::new(-TILE_SIZE, 0.0, 0.0);
                let right_position = transform.translation + Vec3::new(TILE_SIZE, 0.0, 0.0);
                let below_left = left_position + Vec3::new(0.0, -TILE_SIZE, 0.0);
                let below_right = right_position + Vec3::new(0.0, -TILE_SIZE, 0.0);

                let can_move_left = !not_passable.iter().any(|obstacle| {
                    (obstacle.translation - left_position).length() < TILE_SIZE
                        || (obstacle.translation - below_left).length() < TILE_SIZE
                }) && !plague_positions.iter().any(|&other_position| {
                    (other_position - left_position).length() < TILE_SIZE
                        || (other_position - below_left).length() < TILE_SIZE
                });

                let can_move_right = !not_passable.iter().any(|obstacle| {
                    (obstacle.translation - right_position).length() < TILE_SIZE
                        || (obstacle.translation - below_right).length() < TILE_SIZE
                }) && !plague_positions.iter().any(|&other_position| {
                    (other_position - right_position).length() < TILE_SIZE
                        || (other_position - below_right).length() < TILE_SIZE
                });

                if can_move_left {
                    transform.translation = left_position;
                } else if can_move_right {
                    transform.translation = right_position;
                }
            }
        }
    }
}
