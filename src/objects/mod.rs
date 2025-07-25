use bevy::prelude::*;

mod stellar;
pub use stellar::*;

mod time_to_live;
pub use time_to_live::*;

#[derive(Debug, Clone, Copy, Component, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

#[derive(Debug, Clone, Copy, Component, Deref, DerefMut)]
pub struct HalfStepVelocity(pub Vec2);
