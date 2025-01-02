use bevy::diagnostic::LogDiagnosticsPlugin;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;

pub mod systems;
pub mod components;
mod solid_walls;
mod dirt;
mod walls;
mod enemy;
mod player;
mod bomb;
mod rock;
mod falling_bomb;
mod plague;

use systems::*;
use components::*;

use bomb::components::*;
use bomb::resources::*;
use bomb::systems::*;
// use bomb::systems::*;
use dirt::systems::*;
use dirt::components::*;
use enemy::systems::*;
use player::components::*;
use player::systems::*;
use solid_walls::systems::*;
use walls::systems::*;
use rock::systems::*;
use falling_bomb::systems::*;
use plague::systems::*;


fn main() {
    App::new()
    // licznik zebranych bomb
    .init_resource::<BombCount>()
    .add_plugins(DefaultPlugins)
    .add_plugins(FrameTimeDiagnosticsPlugin::default())
    .add_plugins(LogDiagnosticsPlugin::default())
    .add_systems(Startup, spawn_camera)
    .add_systems(Startup, play_background_music)
    .add_systems(Startup, set_background)
    .add_systems(Startup, spawn_player)
    .add_systems(Startup, setup_colision)
    .add_systems(Startup, (spawn_solid_walls_v, spawn_solid_walls_h))
    .add_systems(Startup, spawn_rock)
    .add_systems(Startup, spawn_falling_bomb)
    .add_systems(Startup, spawn_plagueL)
    .add_systems(Startup, spawn_plagueM)
    .add_systems(Startup, spawn_plagueR)
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
    .add_systems(Update, player_hit_enemy)
    .add_systems(Update, rock_movement)
    .add_systems(Update, falling_bomb_movement)
    .add_systems(Update, plague_movement)
    .add_systems(Update, player_push_system)
    .run();
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
            // println!("Wybucham bombę!");

            // Znajdź wszystkie elementy w zasięgu
            let explosion_range = EXPLOSION_RANGE * TILE_SIZE; 

            for (explodable_entity, explodable_transform) in query_explodable.iter_mut() {
                let distance = bomb_transform
                    .translation
                    .distance(explodable_transform.translation);

                if distance <= explosion_range {
                    // Dodaj `Lifetime` do eksplodowalnych elementów
                    commands.entity(explodable_entity).insert(Lifetime {
                        timer: Timer::from_seconds(0.2, TimerMode::Once), // Dajemy im np. 2 sekundy istnienia
                    });
                    // println!(
                    // "Dodano eksplodujący efekt dla encji {:?}, odległość: {:.2}",
                    //     explodable_entity, distance
                    // );
                }
            }
        }
    }
}



// przechodzę przez wszystkie wysadzalne elementy z lifetimami
// fn explodable_lifetime_system(
//     mut commands: Commands,
//     time: Res<Time>,
//     mut query: Query<(Entity, &mut Lifetime, &Explodable)>, // Dodajemy `Explodable` do query
// ) {
//     for (entity, mut lifetime, _explodable) in query.iter_mut() {
//         // Zmniejsz czas życia
//         lifetime.timer.tick(time.delta());
//         if lifetime.timer.finished() {
//             // Usuń element, gdy czas się skończy
//             commands.entity(entity).despawn();
//             println!("Wysadzam wysadzalny element {:?}", entity);
//         }
//     }
// }
