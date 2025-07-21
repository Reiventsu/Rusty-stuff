use raylib::prelude::*;
use std::collections::VecDeque;
use raylib::texture::Image;
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
   texture: Texture2D,
}

impl Food {
    pub fn new(position: Vector2, texture: Texture2D) -> Self {
        Self { position, texture }
    }

    pub fn generate_random_cell(rl: &mut RaylibHandle) -> Vector2 {
        let rand_x = rl.get_random_value::<i32>(0..CELL_COUNT).clamp(0, CELL_COUNT - 1);
        let rand_y = rl.get_random_value::<i32>(0..CELL_COUNT).clamp(0, CELL_COUNT -1);

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

    pub fn load_texture(rl: &mut RaylibHandle, thread: &RaylibThread) -> Texture2D {
        let image = Image::load_image("src/Graphics/FoodImage.png")
            .expect("Could not load image");

        rl.load_texture_from_image(&thread, &image)
            .expect("Could not create texture from image")
    }


    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        let x = OFFSET + self.position.x * CELL_SIZE;
        let y = OFFSET + self.position.y * CELL_SIZE;
        d.draw_texture(
            &self.texture,
            x as i32,
            y as i32,
            Color::WHITE
        );
    }
}