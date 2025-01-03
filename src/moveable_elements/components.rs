use bevy::prelude::*;

#[derive(Component, Copy, Clone)]
pub enum MovableElement {
    PlagueL,
    PlagueM,
    PlagueR,
    FallingBomb,
    Rock,
}
