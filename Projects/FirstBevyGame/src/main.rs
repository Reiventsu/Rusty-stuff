// Modules
mod constants;
mod core;
mod systems;

use crate::core::*;
use crate::systems::*;
use bevy::{input::common_conditions::input_just_released, prelude::*};

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);

    // Physics
    app.insert_resource(Time::<Fixed>::from_hz(60.));

    // Startup systems
    app.add_systems(Startup, (spawn_camera, spawn_map, inti_ui));

    // Update systems
    app.add_systems(
        FixedUpdate,
        (
            apply_velocity,
            apply_gravity.before(apply_velocity),
            bounce.after(apply_velocity),
        ),
    );

    app.add_systems(
        Update,
        (
            player_look,
            player_move.after(player_look),
            focus_events,
            toggle_grab.run_if(input_just_released(KeyCode::Escape)),
            spawn_ball,
            shoot_ball.before(spawn_ball).before(focus_events),
            update_power_bar,
        ),
    );

    // Observers
    app.add_observer(apply_grab);

    // Events
    app.add_event::<BallSpawn>();

    // Resources
    app.init_resource::<BallData>();

    app.insert_resource(Power {
        charging: false,
        current: 0.,
    });
    app.run();
}
// Code
fn spawn_map(mut commands: Commands, ball_data: Res<BallData>) {
    commands.spawn(DirectionalLight::default());
    for h in 0..ball_data.materials.len() {
        commands.spawn((
            Transform::from_translation(Vec3::new(
                (-(ball_data.materials.len() as f32) * 0.5 + h as f32) * 2.,
                0.,
                -50.0,
            )),
            Mesh3d(ball_data.mesh()),
            MeshMaterial3d(ball_data.materials[h].clone()),
        ));
    }
}
