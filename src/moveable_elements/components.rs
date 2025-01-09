use bevy::prelude::*;

#[derive(Component, Copy, Clone)]
pub enum MovableElement {
    FallingBomb,
    Rock,
}
