use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct StellarObject {
    pub name: String,
    pub mass: SolarMass,
    pub last_position: Vec2,
    pub curr_position: Vec2,
}

impl StellarObject {
    pub fn new(name: impl Into<String>, mass: SolarMass, position: Vec2) -> Self {
        Self {
            name: name.into(),
            mass,
            last_position: position,
            curr_position: position,
        }
    }
}

#[derive(Debug, Clone, Copy, Deref)]
pub struct SolarMass(pub f32);
