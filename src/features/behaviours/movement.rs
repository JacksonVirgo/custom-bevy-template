use avian2d::prelude::*;
use bevy::prelude::*;

#[derive(Component, Debug, Default)]
#[require(LinearVelocity)]
pub struct Move {
    pub speed: f32,
    pub dir: Vec2,
}

impl Move {
    pub fn new(speed: f32) -> Self {
        Self { speed, ..default() }
    }
}

pub fn handle_movement(mut q_moving: Query<(&Move, &mut LinearVelocity)>) {
    for (moving, mut linvel) in &mut q_moving {
        *linvel = (moving.dir * moving.speed).normalize_or_zero().into();
    }
}
