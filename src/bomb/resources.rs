use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct BombCount {
    pub value: u32,
}

#[derive(Resource)]
pub struct BombDebounce {
    pub timer: Timer,
}
