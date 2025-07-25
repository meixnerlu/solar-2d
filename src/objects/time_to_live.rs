use bevy::prelude::*;

const TTL: f32 = 120.;

#[derive(Debug, Component)]
pub struct TimeToLive {
    pub ttl: f32,
    pub created_at: f32,
}

impl TimeToLive {
    pub fn new(created_at: f32) -> Self {
        Self {
            created_at,
            ttl: TTL,
        }
    }
}
