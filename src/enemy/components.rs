use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy {
    pub num: i32,
    pub direction: Vec2,
}
