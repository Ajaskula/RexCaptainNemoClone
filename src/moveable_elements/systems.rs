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
        NotPassableForPlayer,
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
        NotPassableForPlayer,
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
        NotPassableForPlayer,
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
        NotPassableForPlayer,
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
        NotPassableForPlayer,
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
        NotPassableForPlayer,
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
        NotPassableForPlayer,
    ));
}

pub fn moveable_elements_movement(
    mut plague_query: Query<(&mut Transform, &MovableElement)>,
    time: Res<Time>,
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


pub fn spawn_rock_on_tower(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Pozycje X wież z funkcji spawn_dirt
    let left_tower_x = 23.0 * TILE_SIZE as f32;
    let right_tower_x = 25.0 * TILE_SIZE as f32;
    let tower_height = 5.0; // Wysokość wieży
    let tower_base_y = -17.0 * TILE_SIZE as f32;

    // Obliczenie pozycji Y szczytu wież
    let tower_top_y = tower_base_y + tower_height * TILE_SIZE;

    // Załadowanie tekstury Rock
    let rock_texture = asset_server.load("textures/rock.png");

    // Tworzenie Rock na szczycie lewej wieży
    commands.spawn((
        Sprite {
            image: rock_texture.clone(),
            custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
            ..Default::default()
        },
        Transform::from_translation(Vec3::new(left_tower_x, tower_top_y, 0.0)),
        MovableElement::Rock, // Komponent Rock
    ));

    // Tworzenie Rock na szczycie prawej wieży
    commands.spawn((
        Sprite {
            image: rock_texture.clone(),
            custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
            ..Default::default()
        },
        Transform::from_translation(Vec3::new(right_tower_x, tower_top_y, 0.0)),
        MovableElement::Rock, // Komponent Rock
    ));
}


// pub fn falling_bomb_explosion(
//     mut commands: Commands,
//     mut falling_bomb_query: Query<(&mut Transform, &mut MovableElement, Entity), With<FallingBomb>>,
//     time: Res<Time>,
//     not_passable: Query<&Transform, With<NotPassableForEnemy>>,
//     mut colision_debounce: ResMut<ColisionDebounce>, // Timer do kontroli częstotliwości kolizji
//     asset_server: Res<AssetServer>,
// ) {
//     for (mut transform, mut movable_element, entity) in falling_bomb_query.iter_mut() {
//         // Sprawdzanie, czy bomba spadła na coś
//         let mut collision = false;
//         let mut has_fallen = false;

//         // Debounce, aby zapobiec wielokrotnemu sprawdzeniu kolizji w krótkim czasie
//         colision_debounce.timer.tick(time.delta());

//         // Sprawdzamy kolizję z innymi obiektami (np. przeszkodami)
//         for obstacle in not_passable.iter() {
//             if (transform.translation.x - obstacle.translation.x).abs() < TILE_SIZE
//                 && (transform.translation.y - obstacle.translation.y).abs() < TILE_SIZE
//             {
//                 collision = true;
//                 break; // Jeśli znajdziemy kolizję, przerywamy
//             }
//         }

//         // Jeśli bomba nie jest w ruchu, nie sprawdzamy jej kolizji
//         if movable_element.is_in_motion {
//             // Sprawdzenie, czy nie leży na czymś już od początku gry (nie wybucha)
//             if !collision && !colision_debounce.timer.finished() {
//                 continue; // Jeśli nie ma kolizji i bomba nie miała jeszcze kontaktu, kontynuujemy
//             }

//             if collision {
//                 // Przestań się ruszać
//                 movable_element.is_in_motion = false; // Zatrzymujemy ruch

//                 // Spawnuje eksplozję
//                 commands.spawn((
//                     Sprite {
//                         image: asset_server.load("textures/explosion.png"), // Tutaj możesz podać odpowiednią teksturę wybuchu
//                         custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
//                         ..Default::default()
//                     },
//                     Transform::from_translation(transform.translation),
//                     Explosion, // Komponent wybuchu
//                 ));

//                 // Zdejmujemy `FallingBomb` z ruchu i usuwamy ją po wybuchu
//                 commands.entity(entity).despawn();

//                 // Resetowanie timera
//                 colision_debounce.timer.reset();
//             }
//         } 
//     }
// }
