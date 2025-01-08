use bevy::math::Vec2;

pub const ENEMY_SPEED: f32 = 50.0;

pub const ENEMY_DIRECTIONS_ARRAY: [Vec2; 4] = [
    Vec2::new(1.0, 0.0),
    Vec2::new(0.0, 1.0),
    Vec2::new(-1.0, 0.0),
    Vec2::new(0.0, -1.0),
];
