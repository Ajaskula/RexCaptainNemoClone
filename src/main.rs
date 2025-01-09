use bevy::diagnostic::LogDiagnosticsPlugin;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;

mod bomb;
pub mod components;
mod dirt;
mod enemy;
mod moveable_elements;
mod player;
mod solid_walls;
pub mod systems;
mod walls;

use components::*;
use systems::*;

use bomb::components::*;
use bomb::resources::*;
use bomb::systems::*;
// use bomb::systems::*;
use dirt::components::*;
use dirt::systems::*;
use enemy::systems::*;
use moveable_elements::systems::*;
use player::components::*;
use player::resources::*;
use player::systems::*;
use solid_walls::systems::*;
use std::time::Duration;
use walls::systems::*;

struct LevelPlugin;
struct GameplayPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_solid_walls)
            .add_systems(Startup, spawn_wall_rectangles)
            .add_systems(Startup, spawn_random_walls)
            .add_systems(Startup, spawn_player)
            .add_systems(Startup, spawn_camera)
            .add_systems(Startup, play_background_music)
            .add_systems(Startup, set_background)
            .add_systems(Startup, setup_collision)
            .add_systems(Startup, spawn_rocks)
            .add_systems(Startup, setup_bomb)
            .add_systems(Startup, spawn_bombs)
            .add_systems(Startup, spawn_enemy)
            .add_systems(Startup, spawn_enemies)
            .add_systems(Startup, spawn_exit)
            .add_systems(Startup, spawn_falling_bomb);
    }
}

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, player_movement)
            .add_systems(Update, update_camera)
            .add_systems(Update, player_dig_dirt)
            .add_systems(Update, player_collect_bomb)
            .add_systems(Update, plant_bomb_system)
            .add_systems(Update, remove_explosion_sprite)
            .add_systems(Update, explosive_lifetime_system)
            .add_systems(Update, explodable_lifetime_system)
            .add_systems(Update, enemy_movement)
            .add_systems(Update, player_hit_enemy)
            .add_systems(Update, moveable_elements_movement)
            .add_systems(Update, player_push_system)
            .add_systems(Update, enemy_hit_moveable_element);
    }
}
fn main() {
    App::new()
        .init_resource::<BombCount>()
        .insert_resource(PlayerMoveCooldown {
            last_move_time: Duration::from_secs(0),
        })
        .insert_resource(PushCooldownTimer::default())
        .add_plugins(DefaultPlugins)
        .add_plugins((LevelPlugin, GameplayPlugin))
        //.add_plugins(FrameTimeDiagnosticsPlugin::default())
        //.add_plugins(LogDiagnosticsPlugin::default())
        .run();
}
