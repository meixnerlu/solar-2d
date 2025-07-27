use bevy::prelude::*;

use crate::{components::*, resources::*};

const G: f32 = 2.9591221e-4;
const TIME_SCALER: f32 = 100.;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, handle_orbital_physics);
    }
}

fn handle_orbital_physics(
    mut render_info: ResMut<RenderInfo>,
    time: Res<Time<Fixed>>,
    query: Query<(&mut Velocity, &mut HalfStepVelocity, &mut StellarObject)>,
) {
    let dt = time.delta_secs() * TIME_SCALER;
    let mut velocities = vec![];
    let mut half_velocities = vec![];
    let mut objects: Vec<Mut<'_, StellarObject>> = vec![];

    for (velocity, half_velocity, object) in query {
        velocities.push(velocity);
        half_velocities.push(half_velocity);
        objects.push(object);
    }
    let object_count = objects.len();

    let accelerations = calculate_accelerations(&mut objects, object_count);

    for a in 0..object_count {
        let acceleration = accelerations[a] * (1.0 / *objects[a].mass);
        **half_velocities[a] = **velocities[a] + (acceleration * dt * 0.5); // KICK
        objects[a].last_position = objects[a].curr_position;
        objects[a].curr_position += **half_velocities[a] * dt; // DRIFT
        objects[a].movement = objects[a].curr_position - objects[a].last_position;
    }

    render_info.time_since_last_update = 0.;

    let accelerations = calculate_accelerations(&mut objects, object_count);

    for a in 0..object_count {
        let acceleration = accelerations[a] * (1.0 / *objects[a].mass);
        **velocities[a] = **half_velocities[a] + (acceleration * dt * 0.5); // KICK
    }
}

fn calculate_accelerations(
    objects: &mut Vec<Mut<'_, StellarObject>>,
    object_count: usize,
) -> Vec<Vec2> {
    let mut accelerations = vec![Vec2::ZERO; object_count];

    for a in 0..object_count {
        for b in (a + 1)..object_count {
            let object_a = &objects[a];
            let object_b = &objects[b];

            let delta = object_b.curr_position - object_a.curr_position;
            let distance = delta.length();
            let direction = delta.normalize();

            let force = G * ((*object_a.mass * *object_b.mass) / distance.powi(2));
            let force_with_direction = direction * force;

            accelerations[a] += force_with_direction;
            accelerations[b] -= force_with_direction;
        }
    }

    accelerations
}
