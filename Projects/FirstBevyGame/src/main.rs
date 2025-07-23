use bevy::input::common_conditions::input_just_released;
use bevy::prelude::*;
use bevy::input::mouse::AccumulatedMouseMotion;
use bevy::window::{CursorGrabMode, PrimaryWindow, WindowFocused};

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    // Startup systems
    app.add_systems(Startup, (spawn_camera, spawn_map));

    // Update systems
    app.add_systems(Update, (
        player_look,
        focus_events,
        toggle_grab.run_if(input_just_released(KeyCode::Escape)),
    ));

    // Observers
    app.add_observer(apply_grab);
    app.run();
}

#[derive(Event, Deref)]
struct GrabEvent(bool);

#[derive(Component)]
struct Player;

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera3d::default(), Player));
}

fn spawn_map(
    mut commands: Commands,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut material_assets: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(DirectionalLight::default());
    let ball_mesh = mesh_assets.add(Sphere::new(1.));
    for h in 0..16 {
        let color = Color::hsl((h as f32 / 16.) * 360., 1., 0.5);
        let ball_material = material_assets.add(StandardMaterial {
            base_color: color,
            ..Default::default()
        });
        commands.spawn((
            Transform::from_translation(
                Vec3::new((-8. + h as f32) * 2., 0., -50.0)),
            Mesh3d(ball_mesh.clone()),
            MeshMaterial3d(ball_material),
        ));
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
    let sensitivity = 100. /
        window.width().min(window.height());
    use EulerRot::YXZ;
    let (mut yaw, mut pitch, _) = player.rotation.to_euler(YXZ);
    pitch -= mouse_motion.delta.y * dt * sensitivity;
    yaw -= mouse_motion.delta.x * dt * sensitivity;
    pitch = pitch.clamp(-1.57, 1.57); // half of pi
    player.rotation = Quat::from_euler(YXZ, yaw, pitch, 0.);
}

// Observers
fn apply_grab(
    grab: Trigger<GrabEvent>,
    mut window: Single<&mut Window, With<PrimaryWindow>>,
) {
    if **grab {
        window.cursor_options.visible = false;
        window.cursor_options.grab_mode = CursorGrabMode::Locked;
    } else {
        window.cursor_options.visible = true;
        window.cursor_options.grab_mode = CursorGrabMode::None
    }
}

fn focus_events(
    mut events: EventReader<WindowFocused>,
    mut commands: Commands,
) {
    if let Some(event) = events.read().last() {
        commands.trigger(GrabEvent(event.focused));
    }
}

fn toggle_grab(
    mut window: Single<&mut Window, With<PrimaryWindow>>,
    mut commands: Commands,
) {
    window.focused = !window.focused;
    commands.trigger(GrabEvent(window.focused));
}