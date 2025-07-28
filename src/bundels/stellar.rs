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
    pub Text2d,
    pub TextFont,
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
        let stellar_object = StellarObject::new(mass, position);
        let stellar_position = Transform::from_xyz(position.x, position.y, 0.).with_scale(Vec3::splat(1. / 30.));
        Self(
            stellar_object,
            velocity,
            HalfStepVelocity(*velocity),
            stellar_position,
            mesh,
            material,
            Text2d("\n".to_string() + &name.into()),
            TextFont {
                font_size: 10.,
                font_smoothing: bevy::text::FontSmoothing::AntiAliased,
                ..Default::default()
            }
        )
    }
}
