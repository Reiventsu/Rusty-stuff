use super::food::Food;
use super::snake::Snake;
use crate::CELL_COUNT;
use raylib::prelude::*;

pub struct Game {
    pub(crate) snake: Snake,
    food: Food,
    pub(crate) game_running: bool,
    pub(crate) game_score: u32,
}

impl Game {
pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
    let snake = Snake::new();
    let food_texture = Food::load_texture(rl, thread);
    let food_position =
        Food::generate_random_positon(rl, &snake.body).unwrap_or(Vector2::new(12.0, 12.0));

    Self {
        snake,
        food: Food::new(food_position, food_texture),
        game_running: true,
        game_score: 0,
    }
}

    pub fn update(&mut self, rl: &mut RaylibHandle) {
        if self.game_running {
            self.snake.update();
            self.check_collision_with_food(rl);
            self.check_collision_with_edges();
            self.check_collision_with_tail();
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        self.snake.draw(d);
        self.food.draw(d);
    }

    fn check_collision_with_food(&mut self, rl: &mut RaylibHandle) {
        if let Some(head) = self.snake.body.front() {
            if *head == self.food.position {
                self.game_score += 1;
                self.snake.should_grow = true;

                if let Ok(new_pos) = Food::generate_random_positon(rl, &self.snake.body) {
                    self.food.position = new_pos;
                }
            }
        }
    }

    fn check_collision_with_edges(&mut self) {
        if let Some(head) = self.snake.body.front() {
            if head.x < 0.0 || head.x >= CELL_COUNT as f32 || head.y < 0.0 || head.y >= CELL_COUNT as f32 {
                self.game_over();
            }
        }
    }

    fn check_collision_with_tail(&mut self) {
        if let Some(head) = self.snake.body.front() {
            let mut body_iter = self.snake.body.iter().skip(1);
            if body_iter.any(|segment| segment == head) {
                self.game_over();
            }
        }
    }

    fn game_over(&mut self) {
        self.game_running = false;
    }

    pub fn reset(&mut self, rl: &mut RaylibHandle) {
        self.snake.reset();
        self.game_running = true;
        self.game_score = 0;
        self.food.position = Food::generate_random_positon(rl, &self.snake.body)
            .unwrap_or(Vector2::new(12.0, 12.0));
    }

}
