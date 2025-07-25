use bevy::math::Vec3;
use bevy::prelude::{Deref, Event};

#[derive(Event)]
pub struct BallSpawn {
    pub position: Vec3,
    pub velocity: Vec3,
    pub power: f32,
}

#[derive(Event, Deref)]
pub struct GrabEvent(pub bool);
