use bevy::prelude::*;
use crate::*;

pub const PLAYER_SPEED: f32 = 5.0;


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
        Explodable{}
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
            let new_x = transform.translation.x + direction.x * PLAYER_SPEED;
            let new_y = transform.translation.y + direction.y * PLAYER_SPEED;

            if (new_x - obstacle.translation.x).abs() < TILE_SIZE 
                && (new_y - obstacle.translation.y).abs() < TILE_SIZE 
            {
                collision = true;
                break; // Nie musimy dalej sprawdzać
            }
        }

        if !collision {
            transform.translation += direction * PLAYER_SPEED;
        }
    }
}
