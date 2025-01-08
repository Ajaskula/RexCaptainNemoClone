use bevy::prelude::*;

#[derive(Component)]
pub struct Lifetime {
    pub timer: Timer,
}

#[derive(Component)]
pub struct NotPassableForEnemy;

#[derive(Component)]
pub struct NotPassableForPlayer;

#[derive(Component)]
pub struct Explosive;

#[derive(Component)]
pub struct Explodable;

#[derive(Component)]
pub struct PlantedBomb;
