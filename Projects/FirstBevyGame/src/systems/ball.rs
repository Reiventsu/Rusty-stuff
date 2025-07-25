use crate::core::{BallData, BallSpawn, Player, Power, Velocity};
use bevy::input::ButtonInput;
use bevy::pbr::MeshMaterial3d;
use bevy::prelude::{
    Commands, EventReader, EventWriter, Mesh3d, MouseButton, Query, Res, ResMut, Single, Time,
    Transform, Window, With,
};

pub fn spawn_ball(
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

pub fn bounce(mut balls: Query<(&Transform, &mut Velocity)>) {
    for (transform, mut velocity) in &mut balls {
        if transform.translation.y < 0. && velocity.y < 0. {
            velocity.y *= -1.;
        }
    }
}
pub fn shoot_ball(
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
