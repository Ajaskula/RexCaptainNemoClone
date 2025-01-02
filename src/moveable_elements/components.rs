use bevy::prelude::*;

#[derive(Component)]
pub enum MovableElement {
    PlagueL,
    PlagueM,
    PlagueR,
    FallingBomb,
    Rock,
}
