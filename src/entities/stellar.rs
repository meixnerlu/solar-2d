use bevy::prelude::*;

use crate::components::*;

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