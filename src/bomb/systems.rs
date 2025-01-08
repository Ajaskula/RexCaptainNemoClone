use bevy::prelude::*;
use rand::Rng;

use crate::bomb::config::BOMB_TIMER_SECONDS;
use crate::bomb::resources::BombDebounce;
use crate::*;

pub const EXPLOSION_RANGE: f32 = 2.0;

// podkłada wrzuca timer dotyczący czasu, po którym można podłożyć następną bombę
pub fn setup_bomb(mut commands: Commands) {
    commands.insert_resource(BombDebounce {
        timer: Timer::from_seconds(0.3, TimerMode::Once), // 0.3 sekundy na debounce
    });
}

// system podkładania bomb
pub fn plant_bomb_system(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut bomb_count: ResMut<BombCount>,       // Zasób licznika bomb
    mut bomb_debounce: ResMut<BombDebounce>, // Timer debouncera
    time: Res<Time>,
    transform_query: Query<&Transform, With<Player>>, // Pobieramy pozycję gracza
    asset_server: Res<AssetServer>,                   // Zasób obrazu bomby
) {
    // Aktualizuj timer, który pozwala na podrzucanie kolejnych bomb
    bomb_debounce.timer.tick(time.delta());

    // sprawdzam czy mogę podrzucić kolejną bombę
    if keyboard_input.pressed(KeyCode::Space)
        && bomb_count.value > 0
        && bomb_debounce.timer.finished()
    {
        // podkładam kolejną bombę
        bomb_count.value -= 1;
        // znajduje pozycje gracza
        if let Ok(transform) = transform_query.get_single() {
            commands // spawnuje bombe w podanej lokalizacji
                .spawn((
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
                    PlantedBomb {},
                )) // dokładam do tego sprite lifetime z timerem
                .insert(Lifetime {
                    timer: Timer::from_seconds(BOMB_TIMER_SECONDS, TimerMode::Once),
                });

            let random_number = rand::thread_rng().gen_range(0..2);

            if random_number == 0 {
                // Spawnuj pierwsze audio
                commands.spawn(AudioPlayer::new(asset_server.load("audio/run.ogg")));
            } else {
                // Spawnuj drugie audio
                commands.spawn(AudioPlayer::new(
                    asset_server.load("audio/explosion_soon.ogg"),
                ));
            }

            // po podłożeniu bomby resetuje timer
            bomb_debounce.timer.reset();
        }
    }
}

// rozkłada bomby na mapie w dosyć losowy sposób
pub fn spawn_bombs(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    // let num = 1;

    let image_wall = asset_server.load("textures/bomb.png");
    // let x = rng.gen_range(0..=(window.width() / TILE_SIZE) as usize) * TILE_SIZE as usize;
    // let y = rng.gen_range(0..=(window.height() / TILE_SIZE) as usize) * TILE_SIZE as usize;
    commands.spawn((
        Sprite {
            image: image_wall.clone(),
            custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
            ..Default::default()
        },
        Transform::from_translation(Vec3::new(
            (TILE_SIZE * -3.0) + window.width() / 2.0,
            TILE_SIZE * 5.0 + window.height() / 2.0,
            0.0,
        )),
        Bomb {},
    ));
    commands.spawn((
        Sprite {
            image: image_wall.clone(),
            custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
            ..Default::default()
        },
        Transform::from_translation(Vec3::new(
            (-TILE_SIZE * 2.0) + window.width() / 2.0,
            TILE_SIZE * 5.0 + window.height() / 2.0,
            0.0,
        )),
        Bomb {},
    ));
    commands.spawn((
        Sprite {
            image: image_wall.clone(),
            custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
            ..Default::default()
        },
        Transform::from_translation(Vec3::new(
            (TILE_SIZE * 1.0) + window.width() / 2.0,
            TILE_SIZE * 5.0 + window.height() / 2.0,
            0.0,
        )),
        Bomb {},
    ));
    commands.spawn((
        Sprite {
            image: image_wall.clone(),
            custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
            ..Default::default()
        },
        Transform::from_translation(Vec3::new(
            (TILE_SIZE * 2.0) + window.width() / 2.0,
            TILE_SIZE * 5.0 + window.height() / 2.0,
            0.0,
        )),
        Bomb {},
    ));
    commands.spawn((
        Sprite {
            image: image_wall.clone(),
            custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
            ..Default::default()
        },
        Transform::from_translation(Vec3::new(
            (TILE_SIZE * 7.0) + window.width() / 2.0,
            TILE_SIZE * 5.0 + window.height() / 2.0,
            0.0,
        )),
        Bomb {},
    ));
}
