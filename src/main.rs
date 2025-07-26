use bevy::{dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin}, prelude::*, render::render_resource::encase::vector::FromVectorParts};

use crate::{components::*, entities::*, plugins::*};

mod components;
mod entities;
mod plugins;

// mass is solar mass
// distance is AU
// Velocity is AU/day
// a second in game is a day * TIME_SCALER is physics

fn main() {
    App::new()
        .insert_resource(Time::<Fixed>::from_hz(48.))
        .add_plugins((DefaultPlugins, PhysicsPlugin, RenderPlugin))
        .add_plugins(
            FpsOverlayPlugin {
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
            },
        )
        .add_systems(Startup, add_solar_system)
        .run();
}

fn add_solar_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Camera2d::default(),
        Projection::Orthographic(OrthographicProjection {
            scale: 1. / 50.,
            ..OrthographicProjection::default_2d()
        }),
    ));

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
}
