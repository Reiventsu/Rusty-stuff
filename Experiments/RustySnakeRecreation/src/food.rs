use raylib::prelude::*;
use std::collections::VecDeque;
use crate::{CELL_COUNT, CELL_SIZE, OFFSET};

// Class for food for the snake to eat
// class Food {

#[derive(Debug, Clone)]
pub struct NoPositionError;

impl std::fmt::Display for NoPositionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "No valid space for food to spawn, failed after 999 attempts.")
    }
}
pub struct Food {
    pub position: Vector2,
}

impl Food {
    pub fn new(position: Vector2) -> Self {
        Self { position }
    }

    pub fn generate_random_cell(rl: &mut RaylibHandle) -> Vector2 {
        let rand_x: i32 = rl.get_random_value(0..CELL_COUNT);
        let rand_y: i32 = rl.get_random_value(0..CELL_COUNT);
        Vector2::new(rand_x as f32, rand_y as f32)
    }

    pub fn generate_random_positon(rl: &mut RaylibHandle, snake_body: &VecDeque<Vector2>) -> Result<Vector2, NoPositionError> {
        let mut attempts = 0;
        const MAX_ATTEMPTS: i32 = 999;

        loop {
            let position = Self::generate_random_cell(rl);
            if !snake_body.contains(&position) {
                return Ok(position);
            }
            attempts += 1;
            if attempts >= MAX_ATTEMPTS {
                return Err(NoPositionError);
            }
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        let x = OFFSET + self.position.x * CELL_SIZE;
        let y = OFFSET + self.position.y * CELL_SIZE;
        d.draw_rectangle(
            x as i32,
            y as i32,
            CELL_SIZE as i32,
            CELL_SIZE as i32,
            Color::RED,
        )
    }
}