use bevy::prelude::*;

use crate::models::*;

#[derive(Resource)]
pub struct ConfigHandle(pub Handle<Config>);

#[derive(Resource, Deref)]
pub struct TimeScaler(pub f32);
