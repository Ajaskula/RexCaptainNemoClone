use bevy::prelude::*;

#[derive(Resource)]
pub struct BombCount {
    pub value: u32,
}

impl Default for BombCount {
    fn default() -> Self {
        BombCount { value: 0 }
    }
}

#[derive(Resource)]
pub struct BombDebounce {
    pub timer: Timer,
}
