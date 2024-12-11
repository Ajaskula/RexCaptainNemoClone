use bevy::prelude::*;

#[derive(Component)]
pub struct Lifetime {
    pub timer: Timer,
}

#[derive(Component)]
pub struct NotWalkable;


#[derive(Component)]
pub struct Explosive {}

#[derive(Component)]
pub struct Explodable{}

#[derive(Component)]
pub struct PlantedBomb{}
