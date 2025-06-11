use std::collections::VecDeque;
use std::fmt;
use std::fmt::Formatter;
use raylib::core::texture::Image;
use raylib::prelude::*;
use crate::{element_in_deque, CELL_COUNT, CELL_SIZE, OFFSET};

// Class for food for the snake to eat
// class Food {

pub struct Food {
    position: Vector2,
    texture: Texture2D,
}


//     static Vector2 GenerateRandomCell() {
//         const int x = GetRandomValue(0, cellCount - 1);
//         const int y = GetRandomValue(0, cellCount - 1);
//         return Vector2{
//             static_cast<float>(x),
//             static_cast<float>(y)
//         };
//     }


//// Error handling
#[derive(Debug, Clone)]
pub struct NoPositionError;

impl fmt::Display for NoPositionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "No valid space for food to spawn, failed after 1000 attempts.")
    }
}

impl Food {
    pub fn generate_random_cell(rl: &mut RaylibHandle) -> Vector2 {
        let rand_x: i32 = rl.get_random_value(0..CELL_COUNT);
        let rand_y: i32 = rl.get_random_value(0..CELL_COUNT);
        Vector2::new(rand_x as f32, rand_y as f32)
    }

    pub fn generate_random_positon(rl: &mut RaylibHandle, snake_body: VecDeque<Vector2>) -> Result<Vector2, NoPositionError> {
        let mut attempts = 0;
        const MAX_ATTEMPTS: i32 = 1000;

        loop {
            let position = Self::generate_random_cell(rl);
            if !element_in_deque(position, snake_body.clone()) {
                return Ok(position);
            }
            attempts += 1;
            if attempts >= MAX_ATTEMPTS {
                return Err(NoPositionError);
            }
        }
    }

    pub fn food_image(&mut self, d: &mut RaylibDrawHandle, rl: &mut RaylibHandle, thread: &RaylibThread, snake_body: VecDeque<Vector2>) {
        let food = Image::load_image("Graphics/FoodImage.png").unwrap();
        self.texture = d.load_texture_from_image(thread, &food)
            .expect("Buy (ANY) gpu >:(");
        self.position = Self::generate_random_positon(rl, snake_body).expect("Failed to find random position, playspace is full?");
    }
    
    pub fn draw_food(&self, d: &mut RaylibDrawHandle) {
        d.draw_texture(
            &self.texture,
            OFFSET as i32 + self.position.x as i32 * CELL_SIZE as i32,
            OFFSET as i32 + self.position.y as i32 * CELL_SIZE as i32,
            Color::WHITE,
        )
        
    }
}
