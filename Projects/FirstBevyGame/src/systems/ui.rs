use bevy::color::Color;
use bevy::prelude::{BackgroundColor, BorderRadius, Commands, EventReader, Node, PositionType, Query, Res, Single, Trigger, UiRect, Val, Window, With};
use bevy::window::{CursorGrabMode, PrimaryWindow, WindowFocused};
use crate::constants::{EMPTY_SPACE, MIN_FILL, NOT_CHARGING};
use crate::core::{GrabEvent, Power, PowerBar};

pub fn inti_ui(mut commands: Commands) {
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
            min_width: Val::VMax(MIN_FILL),
            height: Val::Percent(95.),
            margin: UiRect::all(Val::VMax(0.125)),
            ..Default::default()
        },
                     BackgroundColor(NOT_CHARGING),
                     BorderRadius::all(Val::VMax(5.)),
                     PowerBar { min: 1., max: 6. },
        ));
}

pub fn update_power_bar (
    mut bars: Query<(&mut Node, &PowerBar, &mut BackgroundColor)>,
    power: Res<Power>,
) {
    for (mut bar, config, mut bg) in &mut bars {
        if !power.charging {
            bg.0 = NOT_CHARGING;
            bar.width = Val::VMax(MIN_FILL);
        } else {
            let percent = (power.current - config.min) / (config.max - config.min);
            bg.0 = Color::linear_rgb(1. - percent, percent, 0.);
            bar.width = Val::VMax(MIN_FILL + percent * EMPTY_SPACE);
        }
    }
}

pub fn apply_grab(grab: Trigger<GrabEvent>, mut window: Single<&mut Window, With<PrimaryWindow>>) {
    if **grab {
        window.cursor_options.visible = false;
        window.cursor_options.grab_mode = CursorGrabMode::Locked;
    } else {
        window.cursor_options.visible = true;
        window.cursor_options.grab_mode = CursorGrabMode::None
    }
}

pub fn focus_events(mut events: EventReader<WindowFocused>, mut commands: Commands) {
    if let Some(event) = events.read().last() {
        commands.trigger(GrabEvent(event.focused));
    }
}

pub fn toggle_grab(mut window: Single<&mut Window, With<PrimaryWindow>>, mut commands: Commands) {
    window.focused = !window.focused;
    commands.trigger(GrabEvent(window.focused));
}