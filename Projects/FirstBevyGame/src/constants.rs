use bevy::color::Color;
use bevy::math::Vec3;

// Constant values
pub const GRAVITY: Vec3 = Vec3::new(0., -9.8, 0.);
pub const MOVE_SPEED: f32 = 50.;
pub const NOT_CHARGING: Color = Color::linear_rgb(-1.2, 0.2, 0.2);
pub const MIN_FILL: f32 = 29.75 / 6.;
pub const EMPTY_SPACE: f32 = 29.75 - MIN_FILL;