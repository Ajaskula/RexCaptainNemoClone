use crate::moveable_elements::components::*;
use crate::*;
use bevy::prelude::*;

pub const FALLING_SPEED: f32 = 1.2;
pub const TRESHOLD: f32 = 1.0;

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
    // Załaduj obrazek z zasobów
    let image = asset_server.load(image_path);

    // Przygotuj sprite z załadowanym obrazkiem i rozmiarem
    let sprite = Sprite {
        image: image.clone(),
        custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
        ..Default::default()
    };

    // Przygotuj transform z określoną pozycją
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
    Explosive, // Dodajemy Explosive
) {
    // Wywołanie funkcji get_movable_element
    let (sprite, transform, element, explodable, not_passable_enemy, not_passable_player) =
        get_movable_element(position, image_path, movable_element, asset_server);

    // Zwrócenie krotki z dodatkowym komponentem Explosive
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

pub fn spawn_plague_l(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Parametry dla obiektu
    let position = Vec3::new(13.0 * TILE_SIZE as f32, 3.0 * TILE_SIZE as f32, 0.0);
    let image_path = "textures/plagueL.png";
    let movable_element = MovableElement::PlagueL;
    // Spawnowanie obiektu w grze z przygotowanymi komponentami
    commands.spawn(get_movable_element(
        position,
        image_path,
        movable_element,
        &asset_server,
    ));
}

pub fn spawn_plague_m(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Parametry dla obiektu PlagueM
    let position = Vec3::new(15.0 * TILE_SIZE as f32, 3.0 * TILE_SIZE as f32, 0.0);
    let image_path = "textures/plagueM.png";
    let movable_element = MovableElement::PlagueM;

    // Przygotowanie komponentów i spawnowanie obiektu w grze
    commands.spawn(get_movable_element(
        position,
        image_path,
        movable_element,
        &asset_server,
    ));
}

pub fn spawn_plague_r(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Parametry dla obiektu PlagueR
    let position = Vec3::new(17.0 * TILE_SIZE as f32, 3.0 * TILE_SIZE as f32, 0.0);
    let image_path = "textures/plagueR.png";
    let movable_element = MovableElement::PlagueR;

    // Przygotowanie komponentów i spawnowanie obiektu w grze
    commands.spawn(get_movable_element(
        position,
        image_path,
        movable_element,
        &asset_server,
    ));
}

pub fn spawn_rocks(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Określenie ścieżki do tekstury kamienia
    let image_path = "textures/rock.png";

    // Określenie liczby kolumn i wierszy oraz odległości między kamieniami
    let num_columns = 5; // Liczba kolumn
    let num_rows = 3; // Liczba wierszy

    // Odległość między kamieniami
    let offset_x = TILE_SIZE as f32 * 2.0; // Odstęp w poziomie
    let offset_y = TILE_SIZE as f32 * 2.0; // Odstęp w pionie

    // Przesunięcie o 6 kratek w górę
    let shift_up = TILE_SIZE as f32 * 6.0;

    // Generowanie pozycji kamieni w układzie kolumnowym
    for row in 0..num_rows {
        for col in 0..num_columns {
            let position = Vec3::new(
                (col as f32 * offset_x),            // X pozycja kamienia
                (row as f32 * offset_y) + shift_up, // Y pozycja kamienia przesunięta o 6 kratek w górę
                0.0,                                // Z pozycja (na poziomie 0)
            );
            // Spawnowanie kamienia w wygenerowanej pozycji
            commands.spawn(get_movable_element(
                position,
                image_path,
                MovableElement::Rock,
                &asset_server,
            ));
        }
    }
}

pub fn spawn_falling_bomb(mut commands: Commands, asset_server: Res<AssetServer>) {
    let position = Vec3::new(11.0 * TILE_SIZE as f32, 3.0 * TILE_SIZE as f32, 0.0);
    let image_path = "textures/bomb_falling.png";
    let movable_element = MovableElement::FallingBomb;

    // Spawnowanie obiektu z Explosive
    commands.spawn(get_explosive_movable_element(
        position,
        image_path,
        movable_element,
        &asset_server,
    ));
}

pub fn moveable_elements_movement(
    mut plague_query: Query<(&mut Transform, &MovableElement)>,
    not_passable: Query<&Transform, (With<NotPassableForEnemy>, Without<MovableElement>)>,
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
                    && (transform.translation.y - other_position.y).abs() <= TILE_SIZE + TRESHOLD
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

pub fn spawn_rock_on_tower(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Pozycje X wież z funkcji spawn_dirt
    let left_tower_x = 23.0 * TILE_SIZE as f32;
    let right_tower_x = 25.0 * TILE_SIZE as f32;
    let tower_height = 5.0; // Wysokość wieży
    let tower_base_y = -17.0 * TILE_SIZE as f32;

    // Obliczenie pozycji Y szczytu wież
    let tower_top_y = tower_base_y + tower_height * TILE_SIZE;

    // Parametry do spawnowania
    let image_path = "textures/rock.png";
    let movable_element = MovableElement::Rock;

    // Tworzenie Rock na szczycie lewej wieży
    let position_left = Vec3::new(left_tower_x, tower_top_y, 0.0);
    commands.spawn(get_movable_element(
        position_left,
        image_path,
        movable_element,
        &asset_server,
    ));

    // Tworzenie Rock na szczycie prawej wieży
    let position_right = Vec3::new(right_tower_x, tower_top_y, 0.0);
    commands.spawn(get_movable_element(
        position_right,
        image_path,
        movable_element,
        &asset_server,
    ));
}
