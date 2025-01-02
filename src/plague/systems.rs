use bevy::prelude::*;
use crate::plague::components::*;
use crate::*;

pub const FALLING_SPEED: f32 = 0.6;

pub fn spawn_plagueL(
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
        Plague::L,
        Explodable{},
        NotPassableForEnemy,
        // NotPassableForPlayer,
    ));
}

pub fn spawn_plagueM(
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
        Plague::M,
        Explodable{},
        NotPassableForEnemy,
        // NotPassableForPlayer,
    ));
}

pub fn spawn_plagueR(
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
        Plague::R,
        Explodable{},
        NotPassableForEnemy,
        // NotPassableForPlayer,
    ));
}

pub fn plague_movement(
    mut plague_query: Query<(&mut Transform, &Plague)>,
    time: Res<Time>,
    not_passable: Query<&Transform, (With<NotPassableForEnemy>, Without<Plague>)>,
) {
    for (mut transform, plague) in plague_query.iter_mut() {
        let mut collision = false;
        let direction =  Vec3::new(0.0, -1.0, 0.0);
        let new_position = transform.translation + direction * FALLING_SPEED;

        for obstacle in not_passable.iter() {
            if ((new_position.x - obstacle.translation.x).abs() < TILE_SIZE)
                && ((new_position.y - obstacle.translation.y).abs() <= TILE_SIZE)
            {
                collision = true;
                break;
            }
        }
        if !collision {
            transform.translation = new_position;
        }
    }
}
