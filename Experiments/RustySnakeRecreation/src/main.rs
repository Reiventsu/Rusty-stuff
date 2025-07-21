// Use

use raylib::prelude::*;
use std::collections::VecDeque;

// Modules
mod food;
mod game;
mod snake;

// Global values
pub fn element_in_deque(element: Vector2, deque: &VecDeque<Vector2>) -> bool {
    deque.iter().any(|i| *i == element)
}
mod const_colors {
    use raylib::color::*;

    // [C++] constexpr auto green = Color(173, 204, 96, 255);
    pub const fn green() -> Color {
        Color {
            r: 173,
            g: 204,
            b: 96,
            a: 255,
        }
    }

    // [C++] constexpr auto darkGreen = Color(43, 51, 24, 255);
    pub const fn dark_green() -> Color {
        Color {
            r: 43,
            g: 51,
            b: 24,
            a: 255,
        }
    }
}

const CELL_SIZE: f32 = 30.0;
const CELL_COUNT: i32 = 25;
const OFFSET: f32 = 75.0;

fn main() {
    let (mut rl, thread) = init()
        .size(
            2 * OFFSET as i32 + CELL_SIZE as i32 * CELL_COUNT,
            2 * OFFSET as i32 + CELL_SIZE as i32 * CELL_COUNT,
        )
        .title("RustySnake Recreation")
        .build();

    rl.set_target_fps(60);

    let mut game = game::Game::new(&mut rl, &thread);
    let mut accumulated_time = 0.0;
    const MOVE_INTERVAL: f32 = 0.2;

    while !rl.window_should_close() {
        if rl.is_key_pressed(KeyboardKey::KEY_UP) {
            game.snake.process_input(snake::Direction::Up)
        }
        if rl.is_key_pressed(KeyboardKey::KEY_DOWN) {
            game.snake.process_input(snake::Direction::Down)
        }
        if rl.is_key_pressed(KeyboardKey::KEY_LEFT) {
            game.snake.process_input(snake::Direction::Left)
        }
        if rl.is_key_pressed(KeyboardKey::KEY_RIGHT) {
            game.snake.process_input(snake::Direction::Right)
        }

        if !game.game_running && rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            game.reset(&mut rl);
        }

        let delta_time = rl.get_frame_time();
        accumulated_time += delta_time;

        let speed_multiplier = if rl.is_key_down(KeyboardKey::KEY_SPACE) {
            2.0
        } else {
            1.0
        };

        let effective_interval = MOVE_INTERVAL / speed_multiplier;

        if accumulated_time >= effective_interval {
            game.update(&mut rl);
            accumulated_time -= effective_interval;
        }


        let mut d = rl.begin_drawing(&thread);
        d.clear_background(const_colors::green());

        d.draw_rectangle_lines_ex(
            Rectangle::new(
                OFFSET - 5.0,
                OFFSET - 5.0,
                CELL_SIZE * CELL_COUNT as f32 + 10.0,
                CELL_SIZE * CELL_COUNT as f32 + 10.0,
            ),
            5.0,
            const_colors::dark_green(),
        );
        d.draw_text(
            &format!("{}", game.game_score),
            (OFFSET - 5.0) as i32,
            (OFFSET + CELL_SIZE * CELL_COUNT as f32 + 10.0) as i32,
            40,
            const_colors::dark_green(),
        );

        game.draw(&mut d);

        if !game.game_running {
            let screen_width = d.get_screen_width() as f32;
            let screen_height = d.get_screen_height() as f32;
            d.draw_rectangle(0, 0, screen_width as i32, screen_height as i32, Color::new(0, 0, 0, 150));

            let game_over_text = "GAME OVER!";
            let game_over_width = d.measure_text(game_over_text, 70);
            d.draw_text(
                game_over_text,
                (OFFSET + CELL_SIZE * CELL_COUNT as f32 / 2.0 - game_over_width as f32 / 2.0) as i32,
                (OFFSET + CELL_SIZE * CELL_COUNT as f32 / 2.0 - 120.0) as i32,
                70,
                const_colors::green(),
            );

            let score_text = format!("your score was: {}", game.game_score);
            let score_width = d.measure_text(&score_text, 40);
            d.draw_text(
                &score_text,
                (OFFSET + CELL_SIZE * CELL_COUNT as f32 / 2.0 - score_width as f32 / 2.0) as i32,
                (OFFSET + CELL_SIZE * CELL_COUNT as f32 / 2.0 - 40.0) as i32,
                30,
                const_colors::green(),
            );

            let restart_text = "Press SPACE to restart or ESC to close";
            let restart_width = d.measure_text(restart_text, 30);
            d.draw_text(
                restart_text,
                (OFFSET + CELL_SIZE * CELL_COUNT as f32 / 2.0 - restart_width as f32 / 2.0) as i32,
                (OFFSET + CELL_SIZE * CELL_COUNT as f32 / 2.0 + 20.0) as i32,
                30,
                const_colors::green(),
            );
        }
    }
}