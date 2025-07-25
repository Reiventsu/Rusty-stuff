use crate::constants::GRAVITY;
use crate::core::Velocity;
use bevy::prelude::{Query, Res, Time, Transform};

pub fn apply_velocity(mut objects: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in &mut objects {
        transform.translation += velocity.0 * time.delta_secs();
    }
}

pub fn apply_gravity(mut objects: Query<&mut Velocity>, time: Res<Time>) {
    let g = GRAVITY * time.delta_secs();
    for mut velocity in &mut objects {
        **velocity += g;
    }
}
