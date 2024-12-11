use bevy::prelude::*;
use bevy::window::PrimaryWindow;


pub const PLAYER_SPEED: f32 = 5.0;
pub const ENEMY_SPEED: f32 = 50.0;
pub const BOMB_RANGE: f32 = 3.0;

pub const TILE_SIZE: f32 = 40.0;
pub const NUMBER_EMPTY_TILES:  usize = 10;

const VEC2_ARRAY: [Vec2; 4] = [
    Vec2::new(1.0, 0.0),
    Vec2::new(0.0, 1.0),
    Vec2::new(-1.0, 0.0),
    Vec2::new(0.0, -1.0),
];

fn main() {
    App::new()
    .init_resource::<BombCount>()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, spawn_camera)
    .add_systems(Startup, set_background)
    .add_systems(Startup, spawn_player)
    .add_systems(Startup, (spawn_solid_walls_v, spawn_solid_walls_h))
    // .add_systems(Startup, spawn_walls)
    // .add_systems(Startup, spawn_dirt)
    .add_systems(Startup, spawn_full_dirt_rectangles)
    .add_systems(Startup, spawn_wall_rectangles)
    .add_systems(Startup, setup_bomb)
    .add_systems(Startup, spawn_bombs)
    .add_systems(Startup, spawn_enemies)
    .add_systems(Update, player_movement)
    .add_systems(Update, update_camera)
    .add_systems(Update, player_dig_dirt)
    .add_systems(Update, player_collect_bomb)
    .add_systems(Update, plant_bomb_system)
    .add_systems(Update, explosive_lifetime_system)
    .add_systems(Update, explodable_lifetime_system)
    .add_systems(Update, enemy_movement)
    .run();
}




pub fn spawn_camera(
    mut commands : Commands,
) { 
    println!("Spawning Camera");
    commands.spawn(
        (Camera2d,
    )
    );
}


pub fn set_background(
    mut commands : Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>
) {

    let window = window_query.get_single().unwrap();
    for x in (0..2 * window.width() as usize).step_by(TILE_SIZE as usize) {
        for y in (0..2 * window.height() as usize).step_by(TILE_SIZE as usize) {

            let empty_tile_image: Handle<Image> = asset_server.load("textures/empty.png");
            commands.spawn(
                (Sprite {
                    image: empty_tile_image,
                    custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                    ..Default::default()
                },
                Transform {
                    translation: Vec3::new(x as f32 - window.width(), y as f32 - window.height(), -1.0),
                    ..Default::default()
                }
            )
            );
        }
    }

}

// funkcja doda gracza
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

// odpowiada za movement gracza
pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, (With<Player>, Without<NotWalkable>)>,
    not_walkable: Query<&Transform, With<NotWalkable>>,
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


// wygeneruje obramowanie z niezbijalnej skały
pub fn spawn_solid_walls_h(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>
) {

    let window = window_query.get_single().unwrap();
    for x in (0.. 2 * window.width() as usize + TILE_SIZE as usize).step_by(TILE_SIZE as usize) {
        let solid_wall_image = asset_server.load("textures/solid_wall.png");
        commands.spawn((
            Sprite {
                image:solid_wall_image.clone(),
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                ..Default::default()
            },
            Transform::from_translation(Vec3::new(x as f32 - window.width(), -window.height(), 0.0)),
            GlobalTransform::default(),
            // SolidWall{},
            NotWalkable
        )
        );
        commands.spawn((
            Sprite {
                image:solid_wall_image.clone(),
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                ..Default::default()
            },
            Transform::from_translation(Vec3::new(x as f32 - window.width(), window.height(), 0.0)),
            GlobalTransform::default(),
            // SolidWall{},
            NotWalkable
        )
        );
        
    }
}

pub fn spawn_solid_walls_v(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>
) {

    let window = window_query.get_single().unwrap();
    // println!("height = {}", window.height());
    for y in (0.. 2 * window.height() as usize + TILE_SIZE as usize).step_by(TILE_SIZE as usize) {
        let solid_wall_image = asset_server.load("textures/solid_wall.png");
        commands.spawn((
            Sprite {
                image:solid_wall_image.clone(),
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                ..Default::default()
            },
            Transform::from_translation(Vec3::new(window.width(), y as f32 - window.height(), 0.0)),
            GlobalTransform::default(),
            // SolidWall{},
            NotWalkable
        )
        );
        commands.spawn((
            Sprite {
                image:solid_wall_image.clone(),
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                ..Default::default()
            },
            Transform::from_translation(Vec3::new(-window.width(), y as f32 - window.height(), 0.0)),
            GlobalTransform::default(),
            // SolidWall{},
            NotWalkable
        )
        );
        
    }
}

#[derive(Component)]
pub struct Player {}

#[derive(Resource)]
pub struct BombCount {
    pub value: u32,
}

impl Default for BombCount{
    fn default() -> Self {
        BombCount {
            value: 0
        }
    }
}

#[derive(Component)]
pub struct Bomb {}

#[derive(Component)]
pub struct Camera {}

#[derive(Component)]
pub struct Wall {}

#[derive(Component)]
pub struct PlantedBomb{}

#[derive(Component)]
pub struct SolidWall {}

#[derive(Component)]
pub struct Dirt {}

#[derive(Component)]
pub struct Enemy {
    num: i32,
    direction: Vec2,
}

#[derive(Component)]
pub struct NotWalkable;

#[derive(Component)]
pub struct Lifetime {
    timer: Timer,
}

#[derive(Component)]
pub struct Explosive {}

#[derive(Component)]
pub struct Explodable{}


pub fn update_camera(
    mut camera: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
    player: Query<&Transform, (With<Player>, Without<Camera2d>)>,
    time: Res<Time>,
) {
    let Ok(mut camera) = camera.get_single_mut() else {
        return;
    };

    let Ok(player) = player.get_single() else {
        return;
    };

    let Vec3 { x, y, .. } = player.translation;
    let direction = Vec3::new(x, y, camera.translation.z);

    // Applies a smooth effect to camera movement using stable interpolation
    // between the camera position and the player position on the x and y axes.
    camera
        .translation
        .smooth_nudge(&direction, 2.0, time.delta_secs());
}

// spawn walls
pub fn spawn_walls(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>
){

    let window = window_query.get_single().unwrap();
    let num_walls = 20;
    // let mut rng = rand::thread_rng();
    for x in 0..num_walls {
        let image_wall = asset_server.load("textures/wall.png");
        // let x = rng.gen_range(0..=(window.width() / TILE_SIZE) as usize) * TILE_SIZE as usize;
        // let y = rng.gen_range(0..=(window.height() / TILE_SIZE) as usize) * TILE_SIZE as usize;
        commands.spawn((
            Sprite {
                image:image_wall.clone(),
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                ..Default::default()
            },
            Transform::from_translation(Vec3::new(TILE_SIZE*x as f32 + window.width() / 2.0, window.height() / 2.0, 0.0)),
            // SolidWall{},
            Explodable{},
            NotWalkable,
        )
    );
    }
}

// spawn dirt
pub fn spawn_dirt(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>
){

    let window = window_query.get_single().unwrap();
    let num_walls = 15;
    // let mut rng = rand::thread_rng();
    for x in 0..num_walls {
        let image_bomb = asset_server.load("textures/dirt.png");
        // let x = rng.gen_range(0..=(window.width() / TILE_SIZE) as usize) * TILE_SIZE as usize;
        // let y = rng.gen_range(0..=(window.height() / TILE_SIZE) as usize) * TILE_SIZE as usize;
        commands.spawn((
            Sprite {
                image:image_bomb.clone(),
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                ..Default::default()
            },
            Transform::from_translation(Vec3::new((TILE_SIZE * x as f32)  + window.width() / 2.0, TILE_SIZE* 5.0 + window.height() / 2.0, 0.0)),
            Dirt{},
            Explodable{}
        )
    );
    }
}

pub fn spawn_bombs(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    let window = window_query.get_single().unwrap();
    // let num = 1;

    let image_wall = asset_server.load("textures/bomb.png");
        // let x = rng.gen_range(0..=(window.width() / TILE_SIZE) as usize) * TILE_SIZE as usize;
        // let y = rng.gen_range(0..=(window.height() / TILE_SIZE) as usize) * TILE_SIZE as usize;
        commands.spawn((
            Sprite {
                image:image_wall.clone(),
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                ..Default::default()
            },
            Transform::from_translation(Vec3::new((TILE_SIZE * -3.0)  + window.width() / 2.0, TILE_SIZE* 5.0 + window.height() / 2.0, 0.0)),
            Bomb{},
        ));
        commands.spawn((
            Sprite {
                image:image_wall.clone(),
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                ..Default::default()
            },
            Transform::from_translation(Vec3::new((-TILE_SIZE * 2.0)  + window.width() / 2.0, TILE_SIZE* 5.0 + window.height() / 2.0, 0.0)),
            Bomb{},
        ));
        commands.spawn((
            Sprite {
                image:image_wall.clone(),
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                ..Default::default()
            },
            Transform::from_translation(Vec3::new((TILE_SIZE * 1.0)  + window.width() / 2.0, TILE_SIZE* 5.0 + window.height() / 2.0, 0.0)),
            Bomb{},
        ));
        commands.spawn((
            Sprite {
                image:image_wall.clone(),
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                ..Default::default()
            },
            Transform::from_translation(Vec3::new((TILE_SIZE * 2.0)  + window.width() / 2.0, TILE_SIZE* 5.0 + window.height() / 2.0, 0.0)),
            Bomb{},
        ));
        commands.spawn((
            Sprite {
                image:image_wall.clone(),
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                ..Default::default()
            },
            Transform::from_translation(Vec3::new((TILE_SIZE * 7.0)  + window.width() / 2.0, TILE_SIZE* 5.0 + window.height() / 2.0, 0.0)),
            Bomb{},
        ));

}


#[derive(Resource)]
struct BombDebounce {
    timer: Timer,
}
fn setup_bomb(mut commands: Commands) {
    commands.insert_resource(BombDebounce {
        timer: Timer::from_seconds(0.3, TimerMode::Once), // 0.3 sekundy na debounce
    });
}

fn plant_bomb_system(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut bomb_count: ResMut<BombCount>, // Zasób licznika bomb
    mut bomb_debounce: ResMut<BombDebounce>, // Timer debouncera
    time: Res<Time>,
    transform_query: Query<&Transform, With<Player>>, // Pobieramy pozycję gracza
    asset_server: Res<AssetServer>, // Zasób obrazu bomby
) {
    // Aktualizuj timer debouncera
    bomb_debounce.timer.tick(time.delta());

    if keyboard_input.pressed(KeyCode::Space)
        && bomb_count.value > 0
        && bomb_debounce.timer.finished()
    {
        bomb_count.value -= 1;
        if let Ok(transform) = transform_query.get_single() {
            commands
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
                ))
                .insert(Lifetime {
                    timer: Timer::from_seconds(3.0, TimerMode::Once),
                });

            println!("Plantuje bombę");

            // Resetuj timer debouncera
            bomb_debounce.timer.reset();
        }
    }
}





fn explosive_lifetime_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Lifetime, &Transform), With<PlantedBomb>>, // Encje bombowe z `Lifetime` i `Transform`
    mut query_explodable: Query<(Entity, &Transform), With<Explodable>>, // Encje eksplodowalne z `Transform`
) {
    // przechodzę przez podłożone bomby
    for (entity, mut lifetime, bomb_transform) in query.iter_mut() {
        // Zmniejsz czas życia
        lifetime.timer.tick(time.delta());
        if lifetime.timer.finished() {
            // Usuń bombę, gdy czas się skończy
            commands.entity(entity).despawn();
            println!("Wybucham bombę!");

            // Znajdź wszystkie elementy w zasięgu
            let explosion_range = 3.0 * TILE_SIZE; // Zakładamy, że zasięg wybuchu to 3 jednostki

            for (explodable_entity, explodable_transform) in query_explodable.iter_mut() {
                let distance = bomb_transform
                    .translation
                    .distance(explodable_transform.translation);

                if distance <= explosion_range {
                    // Dodaj `Lifetime` do eksplodowalnych elementów
                    commands.entity(explodable_entity).insert(Lifetime {
                        timer: Timer::from_seconds(0.2, TimerMode::Once), // Dajemy im np. 2 sekundy istnienia
                    });
                    println!(
                        "Dodano eksplodujący efekt dla encji {:?}, odległość: {:.2}",
                        explodable_entity, distance
                    );
                }
            }
        }
    }
}

fn explodable_lifetime_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Lifetime, &Explodable)>, // Dodajemy `Explodable` do query
) {
    for (entity, mut lifetime, _explodable) in query.iter_mut() {
        // Zmniejsz czas życia
        lifetime.timer.tick(time.delta());
        if lifetime.timer.finished() {
            // Usuń element, gdy czas się skończy
            commands.entity(entity).despawn();
            println!("Wysadzam wysadzalny element {:?}", entity);
        }
    }
}


pub fn spawn_enemies(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    let image_enemy = asset_server.load("textures/mumionek.png");
    let tile_size = TILE_SIZE;

    // Lista prostokątów, w których będą spawnować się wrogowie
    let rectangles = vec![
        (5.0, 5.0, 8.0, 6.0),   // (start_x, start_y, width, height)
        (15.0, 10.0, 10.0, 5.0),
    ];

    // Wygenerowanie wroga w każdym prostokącie
    for (start_x, start_y, width, height) in rectangles {
        // Losowa pozycja w obrębie prostokąta (tylko jeden wróg na prostokąt)
        let rand_x = start_x + rand::random::<f32>() * width;
        let rand_y = start_y + rand::random::<f32>() * height;

        // Oblicz współrzędne w przestrzeni świata
        let world_x = rand_x * tile_size - window.width() / 2.0;
        let world_y = rand_y * tile_size - window.height() / 2.0;

        // Tworzymy wroga
        commands.spawn((
            Sprite {
                image: image_enemy.clone(),
                custom_size: Some(Vec2::new(tile_size, tile_size)),
                ..Default::default()
            },
            Transform::from_translation(Vec3::new(world_x, world_y, 0.0)),
            Enemy {
                num: 0,
                direction: Vec2::new(1.0, 0.0),
            },
            Explodable {},  // Dodajemy komponent Explodable, by wróg mógł wybuchnąć
        ));
    }
}




fn enemy_movement(
    mut enemy_query: Query<(&mut Transform, &mut Enemy), Without<NotWalkable>>,
    time: Res<Time>,
    not_walkable: Query<&Transform, With<NotWalkable>>,
) {
    for (mut transform, mut enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        let mut collision = false;

        let time_delta = time.delta_secs();
        let new_position = transform.translation + direction * ENEMY_SPEED * time_delta;

        for obstacle in not_walkable.iter() {
            if (new_position.x - obstacle.translation.x).abs() < TILE_SIZE
                && (new_position.y - obstacle.translation.y).abs() < TILE_SIZE
            {
                collision = true;
                break;
            }
        }

        if !collision {
            transform.translation = new_position;
        } else {

            enemy.num += 1;
            enemy.num %= 4;
            // Obsługa kolizji
            enemy.direction = VEC2_ARRAY[enemy.num as usize];
        }
    }
}





pub fn spawn_wall_rectangles(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    let image_wall = asset_server.load("textures/wall.png");
    let tile_size = TILE_SIZE;

    // Parametry prostokąta
    let rectangles = vec![
        // Pozycja startowa (x, y), szerokość (w) i wysokość (h)
        (5.0, 5.0, 10.0, 6.0),
        (15.0, 10.0, 8.0, 4.0),
    ];

    for (start_x, start_y, width, height) in rectangles {
        for x in 0..(width as usize) {
            for y in 0..(height as usize) {
                // Generuj tylko krawędzie (górną, dolną, lewą i prawą)
                if x == 0 || x == (width as usize - 1) || y == 0 || y == (height as usize - 1) {
                    let world_x = start_x * tile_size + x as f32 * tile_size - window.width() / 2.0;
                    let world_y = start_y * tile_size + y as f32 * tile_size - window.height() / 2.0;

                    commands.spawn((
                        Sprite {
                            image: image_wall.clone(),
                            custom_size: Some(Vec2::new(tile_size, tile_size)),
                            ..Default::default()
                        },
                        Transform::from_translation(Vec3::new(world_x, world_y, 0.0)),
                        Explodable {},
                        NotWalkable,
                    ));
                }
            }
        }
    }
}

pub fn spawn_full_dirt_rectangles(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    let image_dirt = asset_server.load("textures/dirt.png");
    let tile_size = TILE_SIZE;

    // Lista prostokątów do wygenerowania: (start_x, start_y, width, height)
    let rectangles = vec![
        (5.0, 5.0, 8.0, 6.0),
        (15.0, 10.0, 10.0, 5.0),
    ];

    for (start_x, start_y, width, height) in rectangles {
        for x in 0..(width as usize) {
            for y in 0..(height as usize) {
                // Oblicz współrzędne świata dla każdego kafelka
                let world_x = start_x * tile_size + x as f32 * tile_size - window.width();
                let world_y = start_y * tile_size + y as f32 * tile_size - window.height();

                // Tworzenie kafelka
                commands.spawn((
                    Sprite {
                        image: image_dirt.clone(),
                        custom_size: Some(Vec2::new(tile_size, tile_size)),
                        ..Default::default()
                    },
                    Transform::from_translation(Vec3::new(world_x + 20.0*TILE_SIZE, world_y+20.0*TILE_SIZE, 0.0)),
                    Dirt {},          // Oznaczamy jako Dirt
                    Explodable {},    // Może być eksplodowane
                ));
            }
        }
    }
}
