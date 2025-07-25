use bevy::math::Vec3;
use bevy::prelude::{Component, Deref, DerefMut};

// Public components
#[derive(Component)]
pub struct Player;

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(pub Vec3);

#[derive(Component)]
pub struct PowerBar {
    pub min: f32,
    pub max: f32,
}