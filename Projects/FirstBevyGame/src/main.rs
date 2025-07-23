use bevy::{
    input::{common_conditions::input_just_released, mouse::AccumulatedMouseMotion},
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow, WindowFocused},
};

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);

    // Startup systems
    app.add_systems(Startup, (spawn_camera, spawn_map));

    // Update systems
    app.add_systems(
        Update,
        (
            player_look,
            player_move.after(player_look),
            focus_events,
            toggle_grab.run_if(input_just_released(KeyCode::Escape)),
            spawn_ball,
            shoot_ball.before(spawn_ball).before(focus_events),
        ),
    );

    // Observers
    app.add_observer(apply_grab);

    // Events
    app.add_event::<BallSpawn>();

    // Resources
    app.init_resource::<BallData>();

    app.run();
}

// Definitions / Derives
#[derive(Event, Deref)]
struct GrabEvent(bool);

#[derive(Component)]
struct Player;

#[derive(Event)]
struct BallSpawn {
    position: Vec3,
}

#[derive(Resource)]
struct BallData {
    mesh: Handle<Mesh>,
    materials: Vec<Handle<StandardMaterial>>,
    rng: std::sync::Mutex<rand::rngs::StdRng>,
}
impl BallData {
    fn mesh(&self) -> Handle<Mesh> {
        self.mesh.clone()
    }
    fn material(&self) -> Handle<StandardMaterial> {
        use rand::seq::IndexedRandom;
        let mut rng = self.rng.lock().unwrap();
        self.materials.choose(&mut *rng).unwrap().clone()
    }
}

impl FromWorld for BallData {
    fn from_world(world: &mut World) -> Self {
        let mesh = world.resource_mut::<Assets<Mesh>>().add(Sphere::new(1.));
        let mut materials = Vec::new();
        let mut material_assets = world.resource_mut::<Assets<StandardMaterial>>();
        for i in 0..36 {
            let color = Color::hsl((i * 10) as f32, 1., 0.5);
            materials.push(material_assets.add(StandardMaterial {
                base_color: color,
                ..Default::default()
            }));
        }
        BallData {
            mesh,
            materials,
        }
    }
}

// Code
fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera3d::default(), Player));
}

fn spawn_map(
    mut commands: Commands,
    ball_data: Res<BallData>,
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
            Transform::from_translation(Vec3::new((-8. + h as f32) * 2., 0., -50.0)),
            Mesh3d(ball_mesh.clone()),
            MeshMaterial3d(ball_material),
        ));
    }
}

fn spawn_ball(
    mut events: EventReader<BallSpawn>,
    mut commands: Commands,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut mat_assets: ResMut<Assets<StandardMaterial>>,
) {
    for spawn in events.read() {
        commands.spawn((
            Transform::from_translation(spawn.position),
            Mesh3d(mesh_assets.add(Sphere::new(1.))),
            MeshMaterial3d(mat_assets.add(StandardMaterial::default())),
        ));
    }
}

fn shoot_ball(
    input: Res<ButtonInput<MouseButton>>,
    player: Single<&Transform, With<Player>>,
    mut spawner: EventWriter<BallSpawn>,
    window: Single<&Window, With<Window>>,
) {
    if window.cursor_options.visible {
        return;
    }
    if !input.just_pressed(MouseButton::Left) {
        return;
    }
    spawner.write(BallSpawn {
        position: player.translation,
    });
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

const MOVE_SPEED: f32 = 50.;
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
    player.translation += to_move * time.delta_secs() * MOVE_SPEED;
}
