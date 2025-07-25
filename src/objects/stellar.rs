use bevy::prelude::*;

use crate::objects::{HalfStepVelocity, Velocity};

#[derive(Bundle)]
pub struct StellarBundle(
    pub StellarObject,
    pub Velocity,
    pub HalfStepVelocity,
    pub Transform,
    pub Mesh2d,
    pub MeshMaterial2d<ColorMaterial>,
);

impl StellarBundle {
    pub fn new(
        name: impl Into<String>,
        mass: SolarMass,
        position: Vec2,
        velocity: Velocity,
        mesh: Mesh2d,
        material: MeshMaterial2d<ColorMaterial>,
    ) -> Self {
        let stellar_object = StellarObject::new(name.into(), mass, position);
        let stellar_position = Transform::from_xyz(position.x, position.y, 0.);
        Self(
            stellar_object,
            velocity,
            HalfStepVelocity(*velocity),
            stellar_position,
            mesh,
            material,
        )
    }
}

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
