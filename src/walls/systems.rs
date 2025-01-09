use crate::player::config::{INITIAL_SAFE_ZONE_TILE_DISTANCE, PLAYER_STARTING_TILE_POSITION};
use crate::*;
use rand::{thread_rng, Rng};

fn create_wall_sprite(
    image: Handle<Image>,
    position: Vec3,
) -> (
    Sprite,
    Transform,
    Explodable,
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
        Explodable,
        NotPassableForEnemy,
        NotPassableForPlayer,
    )
}

pub fn spawn_random_walls(mut commands: Commands, asset_server: Res<AssetServer>) {
    let image_wall = asset_server.load("textures/wall.png");
    let mut rng = thread_rng();
    for _ in 0..3000 {
        let (mut x, mut y) = (
            rng.gen_range(-WINDOW_WIDTH_TILES + 1..WINDOW_WIDTH_TILES),
            rng.gen_range(-WINDOW_HEIGHT_TILES + 1..WINDOW_HEIGHT_TILES),
        );
        while Vec2::new(x as f32, y as f32).distance(Vec2::new(
            PLAYER_STARTING_TILE_POSITION.0,
            PLAYER_STARTING_TILE_POSITION.1,
        )) < INITIAL_SAFE_ZONE_TILE_DISTANCE as f32
        {
            (x, y) = (
                rng.gen_range(-WINDOW_WIDTH_TILES + 1..WINDOW_WIDTH_TILES),
                rng.gen_range(-WINDOW_HEIGHT_TILES + 1..WINDOW_HEIGHT_TILES),
            );
        }

        commands.spawn(create_wall_sprite(
            image_wall.clone(),
            Vec3::new(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE, 1.0),
        ));
    }
}

pub fn spawn_wall_rectangles(mut commands: Commands, asset_server: Res<AssetServer>) {
    let image_wall = asset_server.load("textures/wall.png");

    let rectangles = [
        (35.0, 35.0, 1, WINDOW_HEIGHT_TILES / 4),
        (35.0, 35.0, WINDOW_WIDTH_TILES / 4, 1),
        (
            35.0 + WINDOW_WIDTH_TILES as f32 / 4.0,
            35.0,
            1,
            WINDOW_HEIGHT_TILES / 4,
        ),
        (
            35.0,
            35.0 + WINDOW_HEIGHT_TILES as f32 / 4.0,
            WINDOW_WIDTH_TILES / 4 + 1,
            1,
        ),
    ];
    for &(start_x, start_y, width, height) in &rectangles {
        let tile_positions = (0..width as usize)
            .flat_map(|x| (0..height as usize).map(move |y| (x, y)))
            .filter(|&(x, y)| {
                x == 0 || x == (width as usize - 1) || y == 0 || y == (height as usize - 1)
            });

        for (x, y) in tile_positions {
            let world_x = start_x * TILE_SIZE + x as f32 * TILE_SIZE - WINDOW_WIDTH;
            let world_y = start_y * TILE_SIZE + y as f32 * TILE_SIZE - WINDOW_HEIGHT;

            commands.spawn(create_wall_sprite(
                image_wall.clone(),
                Vec3::new(world_x, world_y, 1.0),
            ));
        }
    }
}
