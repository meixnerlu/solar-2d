use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Component, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

#[derive(Debug, Clone, Copy, Component, Deref, DerefMut)]
pub struct HalfStepVelocity(pub Vec2);