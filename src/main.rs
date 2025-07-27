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
            YamlAssetPlugin::<PlanetsConfig>::new(&["config/planets.yaml"]),
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
        .add_systems(Startup, (setup, add_solar_system).chain())
        .add_systems(Update, add_solar_system.run_if(in_state(AppState::Loading)))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let planets: Handle<PlanetsConfig> = asset_server.load("config/planets.yaml");
    commands.insert_resource(PlanetConfigHandle(planets));
}

fn add_solar_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    planet_handle: Res<PlanetConfigHandle>,
    planets: Res<Assets<PlanetsConfig>>,
    mut state: ResMut<NextState<AppState>>,
) {
    if let Some(planets) = planets.get(planet_handle.0.id()) {
        commands.spawn((
            Camera2d::default(),
            Projection::Orthographic(OrthographicProjection {
                scale: 1. / 50.,
                ..OrthographicProjection::default_2d()
            }),
        ));

        for planet in &planets.planets {
            let planet_mesh = meshes.add(Circle::new(planet.radius));
            let planet_material = materials.add(Color::linear_rgb(
                planet.color.red / 255.,
                planet.color.green / 255.,
                planet.color.blue / 255.,
            ));
            let planet_position = Vec2::from_slice(&[planet.position.x, planet.position.y]);
            let planet_velocity = Vec2::from_slice(&[planet.velocity.x, planet.velocity.y]);
            let planet = StellarBundle::new(
                &planet.name,
                SolarMass(planet.mass),
                planet_position,
                Velocity(planet_velocity),
                Mesh2d(planet_mesh),
                MeshMaterial2d(planet_material),
            );
            commands.spawn(planet);
        }

        state.set(AppState::Ready);
    }

    /*
    // Sun
    let solar_mesh = meshes.add(Circle::new(0.2));
    let solar_material = materials.add(Color::linear_rgb(255. / 255., 204. / 255., 64. / 255.));
    let sun = StellarBundle::new(
        "Sun",
        SolarMass(1.),
        Vec2::from_parts([0., 0.]),
        Velocity(Vec2::from_parts([0., 0.])),
        Mesh2d(solar_mesh),
        MeshMaterial2d(solar_material),
    );
    commands.spawn(sun);

    // Earth
    let earth_mesh = meshes.add(Circle::new(0.05));
    let earth_material = materials.add(Color::linear_rgb(100. / 255., 149. / 255., 237. / 255.));
    let earth = StellarBundle::new(
        "Earth",
        SolarMass(3.0e-6),
        Vec2::from_parts([1., 0.]),
        Velocity(Vec2::from_parts([0., -1.72e-2])),
        Mesh2d(earth_mesh),
        MeshMaterial2d(earth_material),
    );
    commands.spawn(earth);

    // Mars
    let mars_mesh = meshes.add(Circle::new(0.04));
    let mars_material = materials.add(Color::linear_rgb(188. / 255., 39. / 255., 50. / 255.));
    let mars = StellarBundle::new(
        "Mars",
        SolarMass(3.2e-7),
        Vec2::from_parts([1.52, 0.]),
        Velocity(Vec2::from_parts([0., -1.29e-2])),
        Mesh2d(mars_mesh),
        MeshMaterial2d(mars_material),
    );
    commands.spawn(mars);

    // Jupiter
    let jupiter_mesh = meshes.add(Circle::new(0.09));
    let jupiter_material = materials.add(Color::linear_rgb(210. / 255., 180. / 255., 140. / 255.));
    let jupiter = StellarBundle::new(
        "Jupiter",
        SolarMass(9.54e-4),
        Vec2::from_parts([5.2, 0.]),
        Velocity(Vec2::from_parts([0., -0.00754])),
        Mesh2d(jupiter_mesh),
        MeshMaterial2d(jupiter_material),
    );
    commands.spawn(jupiter);

    // Comet
    let comet_mesh = meshes.add(Circle::new(0.03));
    let comet_material = materials.add(Color::linear_rgb(200. / 255., 200. / 255., 200. / 255.));
    let comet = StellarBundle::new(
        "Comet",
        SolarMass(1.0e-10),
        Vec2::from_parts([1.392, 0.0]),
        Velocity(Vec2::from_parts([0.0, -0.019010182])),
        Mesh2d(comet_mesh),
        MeshMaterial2d(comet_material),
    );
    commands.spawn(comet);
    */
}
