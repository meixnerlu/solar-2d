use bevy::{
    asset::AssetMetaCheck, dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin}, prelude::*
};
use bevy_common_assets::yaml::YamlAssetPlugin;

use crate::{components::*, bundels::*, models::*, plugins::*, resources::*};

mod components;
mod bundels;
mod models;
mod plugins;
mod resources;

// mass is solar mass
// distance is AU
// Velocity is AU/day
// a second in game is a day * TIME_SCALER is physics

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    Loading,
    Ready,
}

fn main() {
    App::new()
        .insert_resource(Time::<Fixed>::from_hz(48.))
        .add_plugins((
            DefaultPlugins.set(AssetPlugin {
                meta_check: AssetMetaCheck::Never,
                ..default()
            }),
            PhysicsPlugin,
            RenderPlugin,
            YamlAssetPlugin::<Config>::new(&["config.yaml"]),
        ))
        .init_state::<AppState>()
        .add_plugins(FpsOverlayPlugin {
            config: FpsOverlayConfig {
                text_config: TextFont {
                    font_size: 24.0,
                    font: default(),
                    font_smoothing: bevy::text::FontSmoothing::default(),
                    ..default()
                },
                text_color: Color::srgb(0.0, 1.0, 0.0),
                refresh_interval: core::time::Duration::from_millis(100),
                enabled: true,
            },
        })
        .add_systems(Startup, load_config)
        .add_systems(Update, setup.run_if(in_state(AppState::Loading)))
        .run();
}

fn load_config(mut commands: Commands, asset_server: Res<AssetServer>) {
    let config: Handle<Config> = asset_server.load("config.yaml");
    commands.insert_resource(ConfigHandle(config));
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    config_handle: Res<ConfigHandle>,
    config: Res<Assets<Config>>,
    mut state: ResMut<NextState<AppState>>,
) {
    if let Some(config) = config.get(config_handle.0.id()) {
        commands.insert_resource(TimeScaler(config.time_scaler));

        commands.spawn((
            Camera2d::default(),
            Projection::Orthographic(OrthographicProjection {
                scale: 1. / config.camera_zoom,
                ..OrthographicProjection::default_2d()
            }),
        ));

        for planet in &config.planets {
            let planet_mesh = meshes.add(Circle::new(planet.radius));
            let planet_material = materials.add(Color::linear_rgb(
                planet.color.red / 255.,
                planet.color.green / 255.,
                planet.color.blue / 255.,
            ));
            let planet_position = Vec2::from_slice(&[planet.position.x, planet.position.y]);
            let planet_velocity = Vec2::from_slice(&[planet.velocity.x, planet.velocity.y]);
            let planet_bundle = StellarBundle::new(
                &planet.name,
                SolarMass(planet.mass),
                planet_position,
                Velocity(planet_velocity),
                Mesh2d(planet_mesh),
                MeshMaterial2d(planet_material),
            );
            let mut entity = commands.spawn(planet_bundle);
            if planet.is_planet {
                entity.insert(PlanetFlag);
            }
        }

        state.set(AppState::Ready);
    }
}
