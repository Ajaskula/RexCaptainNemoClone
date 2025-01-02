use bevy::prelude::*;

#[derive(Resource)]
pub struct ColisionDebounce {
    pub timer: Timer,
}
