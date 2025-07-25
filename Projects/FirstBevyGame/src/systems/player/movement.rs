use bevy::input::ButtonInput;
use bevy::math::Vec3;
use bevy::prelude::{KeyCode, Res, Single, Time, Transform, Window, With};
use bevy::window::PrimaryWindow;
use crate::constants::MOVE_SPEED;
use crate::core::Player;

pub fn player_move(
    mut player: Single<&mut Transform, With<Player>>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    window: Single<&Window, With<PrimaryWindow>>,
) {
    if window.cursor_options.visible {
        return;
    }
    let mut delta = Vec3::ZERO;
    if input.pressed(KeyCode::KeyA) {
        delta.x -= 1.;
    }
    if input.pressed(KeyCode::KeyD) {
        delta.x += 1.;
    }
    if input.pressed(KeyCode::KeyW) {
        delta.z += 1.;
    }
    if input.pressed(KeyCode::KeyS) {
        delta.z -= 1.;
    }

    let forward = player.forward().as_vec3() * delta.z;
    let right = player.right().as_vec3() * delta.x;
    let mut to_move = forward + right;
    to_move.y = 0.;
    to_move = to_move.normalize_or_zero();
    player.translation += to_move * time.delta_secs() * MOVE_SPEED;
}