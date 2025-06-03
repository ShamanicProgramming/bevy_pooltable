use bevy::prelude::*;

#[derive(Component)]
pub struct CueBall;

#[derive(Component)]
pub struct Velocity {
    pub direction: Vec3,
    pub speed: f32,
}
