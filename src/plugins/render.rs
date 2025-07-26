use std::time::Duration;

use bevy::prelude::*;

use crate::components::*;

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DotTimer::default())
            .insert_resource(TtlTimer::default())
            .add_systems(Startup, create_dot_material)
            .add_systems(FixedUpdate, (update_mesh, create_dots, clear_ttls));
    }
}

fn update_mesh(query: Query<(&StellarObject, &mut Transform)>) {
    for (object, mut transform) in query {
        transform.translation.x = object.curr_position.x;
        transform.translation.y = object.curr_position.y;
    }
}

#[derive(Resource)]
struct DotTimer {
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
struct DotMaterial(Handle<ColorMaterial>);

fn create_dot_material(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    let dot_material = materials.add(Color::linear_rgb(1., 1., 1.));
    commands.insert_resource(DotMaterial(dot_material));
}

fn create_dots(
    mut timer: ResMut<DotTimer>,
    time: Res<Time<Fixed>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    dot_material: Res<DotMaterial>,
    query: Query<&StellarObject>,
) {
    let now = time.elapsed_secs();
    timer.elapsed += time.delta();

    if timer.elapsed >= Duration::from_millis(200) {
        timer.elapsed -= Duration::from_millis(200);
        for object in query {
            if object.name == "Sun" {
                continue;
            }
            let dot_mesh = meshes.add(Circle::new(1.0e-2));
            let dot_transform =
                Transform::from_xyz(object.curr_position.x, object.curr_position.y, 1.);
            let material = (*dot_material).clone_weak();
            let ttl = TimeToLive::new(now);
            commands.spawn((
                Mesh2d(dot_mesh),
                MeshMaterial2d(material),
                dot_transform,
                ttl,
            ));
        }
    }
}

#[derive(Resource)]
struct TtlTimer {
    pub elapsed: Duration,
}

impl Default for TtlTimer {
    fn default() -> Self {
        Self {
            elapsed: Duration::ZERO,
        }
    }
}

fn clear_ttls(
    time: Res<Time<Fixed>>,
    mut timer: ResMut<TtlTimer>,
    mut commands: Commands,
    query: Query<(Entity, &TimeToLive)>,
) {
    let now = time.elapsed_secs();
    timer.elapsed += time.delta();

    if timer.elapsed >= Duration::from_millis(200) {
        timer.elapsed -= Duration::from_millis(200);
        for (entity, ttl) in query {
            if ttl.created_at + ttl.ttl < now {
                commands.entity(entity).despawn();
            }
        }
    }
}
