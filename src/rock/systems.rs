use bevy::prelude::*;
use crate::rock::components::*;
use crate::*;

pub const FALLING_SPEED: f32 = 0.6;

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
        Rock{},
        Explodable{},
        NotPassableForEnemy,
        // NotPassableForPlayer,
    ));
}

pub fn rock_movement(
    mut rock_query: Query<(&mut Transform, &mut Rock)>,
    time: Res<Time>,
    not_passable: Query<&Transform, (With<NotPassableForEnemy>, Without<Rock>)>,
) {
    for (mut transform, mut rock) in rock_query.iter_mut() {
        let mut collision = false;
        let new_position = transform.translation + Vec3::new(0.0, -1.0, 0.0) * FALLING_SPEED;
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