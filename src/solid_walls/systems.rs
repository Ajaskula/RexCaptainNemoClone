use crate::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

fn create_solid_wall_sprite(
    image: Handle<Image>,
    position: Vec3,
) -> (
    Sprite,
    Transform,
    GlobalTransform,
    NotPassableForEnemy,
    NotPassableForPlayer,
) {
    (
        Sprite {
            image,
            custom_size: Some(Vec2::splat(TILE_SIZE)),
            ..Default::default()
        },
        Transform::from_translation(position),
        GlobalTransform::default(),
        NotPassableForEnemy,
        NotPassableForPlayer,
    )
}

pub fn spawn_solid_walls(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single();
    let solid_wall_image = asset_server.load("textures/solid_wall.png");
    let half_width = WINDOW_WIDTH;
    let half_height = WINDOW_HEIGHT;

    // Tworzenie poziomych ścian (góra i dół)
    for x in (-half_width as isize..=half_width as isize)
        .step_by(TILE_SIZE as usize)
        .map(|x| x as f32)
    {
        let top_position = Vec3::new(x, half_height, 4.0);
        let bottom_position = Vec3::new(x, -half_height, 4.0);

        commands.spawn(create_solid_wall_sprite(
            solid_wall_image.clone(),
            top_position,
        ));
        commands.spawn(create_solid_wall_sprite(
            solid_wall_image.clone(),
            bottom_position,
        ));
    }

    // Tworzenie pionowych ścian (lewo i prawo)
    for y in (-half_height as isize..=half_height as isize)
        .step_by(TILE_SIZE as usize)
        .map(|y| y as f32)
    {
        let left_position = Vec3::new(-half_width, y, 4.0);
        let right_position = Vec3::new(half_width, y, 4.0);

        commands.spawn(create_solid_wall_sprite(
            solid_wall_image.clone(),
            left_position,
        ));
        commands.spawn(create_solid_wall_sprite(
            solid_wall_image.clone(),
            right_position,
        ));
    }
}
