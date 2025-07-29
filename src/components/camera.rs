use bevy::prelude::*;

#[derive(Component)]
pub struct CameraController {
    pub key_center: KeyCode,
    pub key_down: KeyCode,
    pub key_left: KeyCode,
    pub key_right: KeyCode,
    pub key_up: KeyCode,
    pub key_zoom_in: KeyCode,
    pub key_zoom_out: KeyCode,
    pub move_speed: f32,
    pub zoom_speed: f32,
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            key_center: KeyCode::Space,
            key_down: KeyCode::KeyS,
            key_left: KeyCode::KeyA,
            key_right: KeyCode::KeyD,
            key_up: KeyCode::KeyW,
            key_zoom_in: KeyCode::KeyR,
            key_zoom_out: KeyCode::KeyF,
            move_speed: 800.,
            zoom_speed: 50.,
        }
    }
}
