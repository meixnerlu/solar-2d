use std::time::Duration;

use bevy::prelude::*;

use crate::{components::*, resources::*};

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DotTimer::default())
            .insert_resource(TtlTimer::default())
            .add_systems(Startup, setup_render_resources)
            .add_systems(Update, update_mesh)
            .add_systems(FixedUpdate, (create_dots, clear_ttls));
    }
}

fn update_mesh(
    time: Res<Time>,
    mut render_info: ResMut<RenderInfo>,
    query: Query<(&StellarObject, &mut Transform)>,
) {
    let delta_change = render_info.time_since_last_update / render_info.physics_step;

    for (object, mut transform) in query {
        transform.translation.x = object.last_position.x + object.movement.x * delta_change;
        transform.translation.y = object.last_position.y + object.movement.y * delta_change;
    }
    render_info.time_since_last_update += time.delta_secs();
}



fn setup_render_resources(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    physics_time: Res<Time<Fixed>>,
) {
    let dot_material = materials.add(Color::linear_rgb(1., 1., 1.));
    commands.insert_resource(DotMaterial(dot_material));
    let physics_step = physics_time.timestep().as_secs_f32();
    commands.insert_resource(RenderInfo {
        physics_step,
        time_since_last_update: 0.,
    });
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
                Transform::from_xyz(object.curr_position.x, object.curr_position.y, -1.);
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
