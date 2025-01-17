use bevy::prelude::*;
use std::time::Duration;

#[derive(Resource)]
pub struct CollisionDebounce {
    pub timer: Timer,
}

#[derive(Resource)]
pub struct PlayerMoveCooldown {
    pub last_move_time: Duration,
}

#[derive(Resource)]
pub struct PushCooldownTimer(pub Timer);
