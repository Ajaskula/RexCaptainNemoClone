use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::bomb::config::{BOMB_DEBOUNCE_SECONDS, BOMB_TIMER_SECONDS};
use crate::bomb::resources::BombDebounce;
use crate::*;

pub fn setup_bomb(mut commands: Commands) {
    commands.insert_resource(BombDebounce {
        timer: Timer::from_seconds(BOMB_DEBOUNCE_SECONDS, TimerMode::Once),
    });
}

pub fn plant_bomb_system(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut bomb_count: ResMut<BombCount>,
    mut bomb_debounce: ResMut<BombDebounce>,
    time: Res<Time>,
    player_position: Query<&Transform, With<Player>>,
    asset_server: Res<AssetServer>,
) {
    bomb_debounce.timer.tick(time.delta());

    if keyboard_input.pressed(KeyCode::Space)
        && bomb_count.value > 0
        && bomb_debounce.timer.finished()
    {
        if let Ok(transform) = player_position.get_single() {
            commands.spawn((
                PlantedBomb,
                Sprite {
                    image: asset_server.load("textures/bomb.png").clone(),
                    custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                    ..Default::default()
                },
                Transform::from_translation(Vec3::new(
                    transform.translation.x,
                    transform.translation.y,
                    0.0,
                )),
                Lifetime {
                    timer: Timer::from_seconds(BOMB_TIMER_SECONDS, TimerMode::Once),
                },
            ));
            bomb_count.value -= 1;

            let random_number = rand::thread_rng().gen_range(0..2);
            if random_number == 0 {
                commands.spawn(AudioPlayer::new(asset_server.load("audio/run.ogg")));
            } else {
                commands.spawn(AudioPlayer::new(
                    asset_server.load("audio/explosion_soon.ogg"),
                ));
            }

            bomb_debounce.timer.reset();
        }
    }
}

pub fn spawn_bombs(mut commands: Commands, asset_server: Res<AssetServer>) {
    let image_wall = asset_server.load("textures/bomb.png");
    // let x = rng.gen_range(0..=(WINDOW_WIDTH / TILE_SIZE) as usize) * TILE_SIZE as usize;
    // let y = rng.gen_range(0..=(WINDOW_HEIGHT / TILE_SIZE) as usize) * TILE_SIZE as usize;
    let xy_around_player = [
        (1, 0),
        (0, 1),
        (1, 1),
        (-1, 0),
        (0, -1),
        (-1, -1),
        (1, -1),
        (-1, 1),
    ];
    for (x, y) in xy_around_player {
        spawn_bomb(&mut commands, image_wall.clone(), x, y);
    }

    let mut rng = thread_rng();
    for _ in 0..1000 {
        let (x, y) = (
            rng.gen_range(-WINDOW_WIDTH_TILES + 1..WINDOW_WIDTH_TILES),
            rng.gen_range(-WINDOW_HEIGHT_TILES + 1..WINDOW_HEIGHT_TILES),
        );
        spawn_bomb(&mut commands, image_wall.clone(), x, y);
    }
}

fn spawn_bomb(mut commands: &mut Commands, image: Handle<Image>, x: i32, y: i32) {
    commands.spawn((
        Bomb {},
        Sprite {
            image,
            custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
            ..Default::default()
        },
        Transform::from_translation(Vec3::new(TILE_SIZE * x as f32, TILE_SIZE * y as f32, 0.0)),
    ));
}
