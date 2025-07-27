#[derive(serde::Deserialize, bevy::asset::Asset, bevy::reflect::TypePath)]
pub struct PlanetsConfig {
    pub planets: Vec<PlanetConfig>,
}

#[derive(serde::Deserialize, bevy::asset::Asset, bevy::reflect::TypePath)]
pub struct PlanetConfig {
    pub name: String,
    pub mass: f32,
    pub position: Vec2Config,
    pub velocity: Vec2Config,
    pub radius: f32,
    pub color: ColorConfig,
}

#[derive(serde::Deserialize, bevy::asset::Asset, bevy::reflect::TypePath)]
pub struct Vec2Config {
    pub x: f32,
    pub y: f32,
}

#[derive(serde::Deserialize, bevy::asset::Asset, bevy::reflect::TypePath)]
pub struct ColorConfig {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}
