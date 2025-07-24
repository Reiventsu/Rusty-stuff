pub mod definitions;

use crate::definitions::*;
use bevy::{
    input::{common_conditions::input_just_released, mouse::AccumulatedMouseMotion},
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow, WindowFocused},
};

//// CONSTANTS
mod constants {
    use bevy::color::Color;
    use bevy::math::Vec3;
    pub const GRAVITY: Vec3 = Vec3::new(0., -9.8, 0.);
    pub const MOVE_SPEED: f32 = 50.;
    pub const NOT_CHARGING: Color = Color::linear_rgb(0.2, 0.2, 0.2);
    pub const MIN_FILL: f32 = 29.75 / 6.;
    pub const EMPTY_SPACE: f32 = 29.75 - MIN_FILL;
}

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

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera3d::default(), Player));
}

fn inti_ui(mut commands: Commands) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                width: Val::VMax(30.),
                height: Val::VMax(5.),
                bottom: Val::Px(20.),
                left: Val::Px(20.),
                ..Default::default()
            },
            BackgroundColor(Color::linear_rgb(0.5, 0.5, 0.5)),
            BorderRadius::all(Val::VMax(5.)),
        ))
        .with_child((Node {
            position_type: PositionType::Absolute,
            min_width: Val::VMax(constants::MIN_FILL),
            height: Val::Percent(95.),
            margin: UiRect::all(Val::VMax(0.125)),
            ..Default::default()
        },
        BackgroundColor(constants::NOT_CHARGING),
            BorderRadius::all(Val::VMax(5.)),
            PowerBar { min: 1., max: 6. },
        ));
}

fn update_power_bar (
    mut bars: Query<(&mut Node, &PowerBar, &mut BackgroundColor)>,
    power: Res<Power>,
) {
    for (mut bar, config, mut bg) in &mut bars {
        if !power.charging {
            bg.0 = constants::NOT_CHARGING;
            bar.width = Val::VMax(constants::MIN_FILL);
        } else {
            let percent = (power.current - config.min) / (config.max - config.min);
            bg.0 = Color::linear_rgb(1. - percent, percent, 0.);
            bar.width = Val::VMax(constants::MIN_FILL + percent * constants::EMPTY_SPACE);
        }
    }
}

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

fn spawn_ball(
    mut events: EventReader<BallSpawn>,
    mut commands: Commands,
    ball_data: Res<BallData>,
) {
    for spawn in events.read() {
        commands.spawn((
            Transform::from_translation(spawn.position),
            Mesh3d(ball_data.mesh()),
            MeshMaterial3d(ball_data.material()),
            Velocity(spawn.velocity * spawn.power * 10.),
        ));
    }
}

fn bounce(mut balls: Query<(&Transform, &mut Velocity)>) {
    for (transform, mut velocity) in &mut balls {
        if transform.translation.y < 0. && velocity.y < 0. {
            velocity.y *= -1.;
        }
    }
}
fn shoot_ball(
    input: Res<ButtonInput<MouseButton>>,
    player: Single<&Transform, With<Player>>,
    mut spawner: EventWriter<BallSpawn>,
    window: Single<&Window, With<Window>>,
    mut power: ResMut<Power>,
    time: Res<Time>,
) {
    if window.cursor_options.visible {
        return;
    }

    if power.charging {
        if input.just_released(MouseButton::Left) {
            spawner.write(BallSpawn {
                position: player.translation,
                velocity: player.forward().as_vec3(),
                power: power.current,
            });
        }
        if input.pressed(MouseButton::Left) {
            power.current += time.delta_secs();
            power.current = power.current.clamp(1., 6.);
        } else {
            power.charging = false;
        }
    }
    if input.just_pressed(MouseButton::Left) {
        power.charging = true;
        power.current = 1.;
    }
}

fn apply_velocity(mut objects: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in &mut objects {
        transform.translation += velocity.0 * time.delta_secs();
    }
}

fn apply_gravity(mut objects: Query<&mut Velocity>, time: Res<Time>) {
    let g = constants::GRAVITY * time.delta_secs();
    for mut velocity in &mut objects {
        **velocity += g;
    }
}

fn player_look(
    mut player: Single<&mut Transform, With<Player>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    time: Res<Time>,
    window: Single<&Window, With<PrimaryWindow>>,
) {
    let dt = time.delta_secs();
    if !window.focused {
        return;
    }
    let sensitivity = 100. / window.width().min(window.height());
    use EulerRot::YXZ;
    let (mut yaw, mut pitch, _) = player.rotation.to_euler(YXZ);
    pitch -= mouse_motion.delta.y * dt * sensitivity;
    yaw -= mouse_motion.delta.x * dt * sensitivity;
    pitch = pitch.clamp(-1.57, 1.57); // half of pi
    player.rotation = Quat::from_euler(YXZ, yaw, pitch, 0.);
}

// Observers
fn apply_grab(grab: Trigger<GrabEvent>, mut window: Single<&mut Window, With<PrimaryWindow>>) {
    if **grab {
        window.cursor_options.visible = false;
        window.cursor_options.grab_mode = CursorGrabMode::Locked;
    } else {
        window.cursor_options.visible = true;
        window.cursor_options.grab_mode = CursorGrabMode::None
    }
}

fn focus_events(mut events: EventReader<WindowFocused>, mut commands: Commands) {
    if let Some(event) = events.read().last() {
        commands.trigger(GrabEvent(event.focused));
    }
}

fn toggle_grab(mut window: Single<&mut Window, With<PrimaryWindow>>, mut commands: Commands) {
    window.focused = !window.focused;
    commands.trigger(GrabEvent(window.focused));
}

fn player_move(
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
    player.translation += to_move * time.delta_secs() * constants::MOVE_SPEED;
}
