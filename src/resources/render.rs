use std::time::Duration;

use bevy::prelude::*;

#[derive(Resource)]
pub struct RenderInfo {
    pub physics_step: f32,
    pub time_since_last_update: f32,
}

#[derive(Resource)]
pub struct DotTimer {
    pub elapsed: Duration,
}

impl Default for DotTimer {
    fn default() -> Self {
        Self {
            elapsed: Duration::ZERO,
        }
    }
}

#[derive(Resource, Deref)]
pub struct DotMesh(pub Handle<Mesh>);

#[derive(Resource, Deref)]
pub struct DotMaterial(pub Handle<ColorMaterial>);

#[derive(Resource)]
pub struct TtlTimer {
    pub elapsed: Duration,
}

impl Default for TtlTimer {
    fn default() -> Self {
        Self {
            elapsed: Duration::ZERO,
        }
    }
}
