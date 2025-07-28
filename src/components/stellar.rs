use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct StellarObject {
    pub mass: SolarMass,
    pub last_position: Vec2,
    pub curr_position: Vec2,
    pub movement: Vec2,
}

impl StellarObject {
    pub fn new(mass: SolarMass, position: Vec2) -> Self {
        Self {
            mass,
            last_position: position,
            curr_position: position,
            movement: Vec2::ZERO,
        }
    }
}

#[derive(Debug, Clone, Copy, Deref)]
pub struct SolarMass(pub f32);
