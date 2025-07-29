use bevy::prelude::*;

use crate::components::*;

pub struct CameraControllPlugin;

impl Plugin for CameraControllPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_camera_controll);
    }
}

fn handle_camera_controll(
    time: Res<Time>,
    key_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Projection, &CameraController), With<Camera2d>>,
) {
    let dt = time.delta_secs();

    let Ok((mut transform, mut projection, controller)) = query.single_mut() else {
        return;
    };
    let mut movement = Vec3::ZERO;
    let Projection::Orthographic(ref mut ortho) = *projection else {
        return;
    };
    if key_input.pressed(controller.key_center) {
        transform.translation.x = 0.;
        transform.translation.y = 0.;
    }
    if key_input.pressed(controller.key_down) {
        movement.y -= 1.;
    }
    if key_input.pressed(controller.key_left) {
        movement.x -= 1.;
    }
    if key_input.pressed(controller.key_right) {
        movement.x += 1.;
    }
    if key_input.pressed(controller.key_up) {
        movement.y += 1.;
    }
    if key_input.pressed(controller.key_zoom_in) {
        movement.z += 1.;
    }
    if key_input.pressed(controller.key_zoom_out) {
        movement.z -= 1.;
    }
    if movement == Vec3::ZERO {
        return;
    }
    movement = movement.normalize();

    transform.translation.x += movement.x * ortho.scale * dt * controller.move_speed;
    transform.translation.y += movement.y * ortho.scale * dt * controller.move_speed;

    let new_zoom = (1. / ortho.scale) + (movement.z * dt * controller.zoom_speed);
    match new_zoom < 0. {
        true => ortho.scale = 1. / 0.01,
        false => ortho.scale = 1. / new_zoom,
    }
}
