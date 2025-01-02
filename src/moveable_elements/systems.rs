use bevy::prelude::*;
use crate::moveable_elements::components::*;
use crate::*;

pub const FALLING_SPEED: f32 = 0.6;
pub const TRESHOLD: f32 = 1.0;

pub fn spawn_plague_l(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
){

    let image_rock = asset_server.load("textures/plagueL.png");
    commands.spawn((
        Sprite {
            image:image_rock.clone(),
            custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
            ..Default::default()
        },
        Transform::from_translation(Vec3::new(13.0 * TILE_SIZE as f32, 3.0 * TILE_SIZE as f32, 0.0)),
        MovableElement::PlagueL,
        Explodable{},
        NotPassableForEnemy,
        // NotPassableForPlayer,
    ));
}

pub fn spawn_plague_m(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
){

    let image_rock = asset_server.load("textures/plagueM.png");
    commands.spawn((
        Sprite {
            image:image_rock.clone(),
            custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
            ..Default::default()
        },
        Transform::from_translation(Vec3::new(15.0 * TILE_SIZE as f32, 3.0 * TILE_SIZE as f32, 0.0)),
        MovableElement::PlagueM,
        Explodable{},
        NotPassableForEnemy,
        // NotPassableForPlayer,
    ));
}

pub fn spawn_plague_r(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
){

    let image_rock = asset_server.load("textures/plagueR.png");
    commands.spawn((
        Sprite {
            image:image_rock.clone(),
            custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
            ..Default::default()
        },
        Transform::from_translation(Vec3::new(17.0 * TILE_SIZE as f32, 3.0 * TILE_SIZE as f32, 0.0)),
        MovableElement::PlagueR,
        Explodable{},
        NotPassableForEnemy,
        // NotPassableForPlayer,
    ));
}

pub fn spawn_rock(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
){

    let image_rock = asset_server.load("textures/rock.png");
    commands.spawn((
        Sprite {
            image:image_rock.clone(),
            custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
            ..Default::default()
        },
        Transform::from_translation(Vec3::new(9.0 * TILE_SIZE as f32, 3.0 * TILE_SIZE as f32, 0.0)),
        MovableElement::Rock,
        Explodable{},
        NotPassableForEnemy,
        // NotPassableForPlayer,
    ));
    commands.spawn((
        Sprite {
            image:image_rock.clone(),
            custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
            ..Default::default()
        },
        Transform::from_translation(Vec3::new(20.0 * TILE_SIZE as f32, 3.0 * TILE_SIZE as f32, 0.0)),
        MovableElement::Rock,
        Explodable{},
        NotPassableForEnemy,
        // NotPassableForPlayer,
    ));
    commands.spawn((
        Sprite {
            image:image_rock.clone(),
            custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
            ..Default::default()
        },
        Transform::from_translation(Vec3::new(20.0 * TILE_SIZE as f32, 4.0 * TILE_SIZE as f32, 0.0)),
        MovableElement::Rock,
        Explodable{},
        NotPassableForEnemy,
        // NotPassableForPlayer,
    ));
    // commands.spawn((
    //     Sprite {
    //         image:image_rock.clone(),
    //         custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
    //         ..Default::default()
    //     },
    //     Transform::from_translation(Vec3::new(24.0 * TILE_SIZE as f32, 3.0 * TILE_SIZE as f32, 0.0)),
    //     MovableElement::Rock,
    //     Explodable{},
    //     NotPassableForEnemy,
    //     // NotPassableForPlayer,
    // ));
    // commands.spawn((
    //     Sprite {
    //         image:image_rock.clone(),
    //         custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
    //         ..Default::default()
    //     },
    //     Transform::from_translation(Vec3::new(24.0 * TILE_SIZE as f32, 4.0 * TILE_SIZE as f32, 0.0)),
    //     MovableElement::Rock,
    //     Explodable{},
    //     NotPassableForEnemy,
    //     // NotPassableForPlayer,
    // ));
}



pub fn spawn_falling_bomb(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
){

    let image_rock = asset_server.load("textures/bomb_falling.png");
    commands.spawn((
        Sprite {
            image:image_rock.clone(),
            custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
            ..Default::default()
        },
        Transform::from_translation(Vec3::new(11.0 * TILE_SIZE as f32, 3.0 * TILE_SIZE as f32, 0.0)),
        MovableElement::FallingBomb,
        Explodable{},
        NotPassableForEnemy,
        // NotPassableForPlayer,
    ));
}

pub fn moveable_elements_movement(
    mut plague_query: Query<(&mut Transform, &MovableElement)>,
    time: Res<Time>,
    not_passable: Query<&Transform, (With<NotPassableForEnemy>, Without<MovableElement>)>,
) {
    // Zbierz pozycje wszystkich elementów `MovableElement`
    let plague_positions: Vec<Vec3> = plague_query
        .iter()
        .map(|(transform, _)| transform.translation)
        .collect();

    // Iteruj przez `plague_query` z numeracją indeksów
    for (index, (mut transform, _)) in plague_query.iter_mut().enumerate() {
        let mut collision = false;
        let direction = Vec3::new(0.0, -1.0, 0.0);
        let new_position = transform.translation + direction * FALLING_SPEED;

        // Sprawdź kolizję z `NotPassableForEnemy`
        for obstacle in not_passable.iter() {
            if ((new_position.x - obstacle.translation.x).abs() < TILE_SIZE)
                && ((new_position.y - obstacle.translation.y).abs() <= TILE_SIZE)
            {
                collision = true;
                println!(
                    "Kolizja z przeszkodą: Element na pozycji {:?} z przeszkodą na {:?}",
                    new_position, obstacle.translation
                );
                break;
            }
        }

        // Sprawdź kolizję z innymi `MovableElement`, ale nie z sobą samym
        for (other_index, other_position) in plague_positions.iter().enumerate() {
            if index != other_index
                && (new_position.x - other_position.x).abs() < TILE_SIZE
                && (new_position.y - other_position.y).abs() <= TILE_SIZE
            {
                collision = true;
                println!(
                    "Kolizja z innym elementem: Element na pozycji {:?} i element na pozycji {:?}",
                    new_position, other_position
                );
                break;
            }
        }

        // Jeśli nie ma kolizji, przesuń element
        if !collision {
            transform.translation = new_position;
        } else {

            println!("sprawdzam czy da się przesunąć w bok");
            // Jeśli element jest ustawiony nad innym, przesuń go w bok
            let mut is_stacked = false;
            for (other_index, other_position) in plague_positions.iter().enumerate() {
                // Obliczanie wartości logicznych dla poszczególnych części warunku
                let x_check = (transform.translation.x - other_position.x).abs() < TILE_SIZE;
                let y_check = (transform.translation.y - other_position.y).abs() <= TILE_SIZE + TRESHOLD;
                let is_above = transform.translation.y > other_position.y;
            
                // Wypisywanie pozycji porównywanych elementów oraz wyników części warunku
                println!(
                    "Porównywanie elementu o pozycji {:?} (index {}) z elementem o pozycji {:?} (index {}):",
                    transform.translation, index, other_position, other_index
                );
                println!(
                    "  x_check = {} (|{} - {}| = {})",
                    x_check,
                    transform.translation.x,
                    other_position.x,
                    (transform.translation.x - other_position.x).abs()
                );
                println!(
                    "  y_check = {} (|{} - {}| = {})",
                    y_check,
                    transform.translation.y,
                    other_position.y,
                    (transform.translation.y - other_position.y).abs()
                );
                println!(
                    "  is_above = {} ({} > {})",
                    is_above,
                    transform.translation.y,
                    other_position.y
                );
            
                // Sprawdzenie głównego warunku
                if index != other_index
                    && x_check
                    && y_check
                    && is_above
                {
                    is_stacked = true;
                    println!(
                        "Element na pozycji {:?} jest ustawiony nad elementem na pozycji {:?}",
                        transform.translation, other_position
                    );
                    break;
                }
            }
            
            

            // Jeśli element jest ustawiony nad innym, przesuń go w bok
            if is_stacked {
                let left_position = transform.translation + Vec3::new(-TILE_SIZE, 0.0, 0.0);
                let right_position = transform.translation + Vec3::new(TILE_SIZE, 0.0, 0.0);

                let can_move_left = !not_passable.iter().any(|obstacle| {
                    (obstacle.translation - left_position).length() < TILE_SIZE
                }) && !plague_positions.iter().any(|&other_position| {
                    (other_position - left_position).length() < TILE_SIZE
                });

                let can_move_right = !not_passable.iter().any(|obstacle| {
                    (obstacle.translation - right_position).length() < TILE_SIZE
                }) && !plague_positions.iter().any(|&other_position| {
                    (other_position - right_position).length() < TILE_SIZE
                });

                // Przesuń w lewo lub w prawo, jeśli możliwe
                if can_move_left {
                    transform.translation = left_position;
                    println!(
                        "Element na pozycji {:?} przesunięty na lewo na {:?}",
                        transform.translation, left_position
                    );
                } else if can_move_right {
                    transform.translation = right_position;
                    println!(
                        "Element na pozycji {:?} przesunięty na prawo na {:?}",
                        transform.translation, right_position
                    );
                }
            }
        }
    }
}
