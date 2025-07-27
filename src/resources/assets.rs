use bevy::prelude::*;

use crate::models::PlanetsConfig;

#[derive(Resource)]
pub struct PlanetConfigHandle(pub Handle<PlanetsConfig>);
