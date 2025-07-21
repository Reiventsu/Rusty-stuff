use crate::{const_colors, CELL_SIZE, OFFSET};
use raylib::prelude::*;
use std::cmp::PartialEq;
use std::collections::VecDeque;
use std::ops::Add;

//// Snake Direction control struct
#[derive(PartialEq, Clone, Copy)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn inverse_direction(self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
        }
    }
    pub fn to_vector(self) -> Vector2 {
        match self {
            Direction::Up => Vector2::new(0.0, -1.0),
            Direction::Right => Vector2::new(1.0, 0.0),
            Direction::Down => Vector2::new(0.0, 1.0),
            Direction::Left => Vector2::new(-1.0, 0.0),
        }
    }
}

//// Snake logic struct
pub struct Snake {
    pub(crate) body: VecDeque<Vector2>,
    direction: Direction,
    next_direction: Direction,
    pub(crate) should_grow: bool,
}

impl Snake {
    const SNAKE_BODY_START: [Vector2; 3] = [
        Vector2::new(6.0, 9.0),
        Vector2::new(5.0, 9.0),
        Vector2::new(4.0, 9.0),
    ];

    pub fn new() -> Self {
        Self {
            body: VecDeque::from(Self::SNAKE_BODY_START),
            direction: Direction::Right,
            next_direction: Direction::Right,
            should_grow: false,
        }
    }

    pub fn process_input(&mut self, new_direction: Direction) {
        if new_direction != self.direction.inverse_direction() {
            self.next_direction = new_direction;
        }
    }

    pub fn update(&mut self) {
        self.direction = self.next_direction;
        let movement = self.direction.to_vector();
        let new_head = self.body.front().unwrap().add(movement);
        self.body.push_front(new_head);

        if !self.should_grow {
            self.body.pop_back();
        } else {
            self.should_grow = false;
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        for segment in &self.body {
            let rect = Rectangle::new(
                OFFSET + segment.x * CELL_SIZE,
                OFFSET + segment.y * CELL_SIZE,
                CELL_SIZE,
                CELL_SIZE,
            );
            d.draw_rectangle_rounded(rect, 0.5, 6, const_colors::dark_green());
        }
    }

    pub fn reset(&mut self) {
        self.body = VecDeque::from(Self::SNAKE_BODY_START);
        self.direction = Direction::Right;
        self.next_direction = Direction::Right;
        self.should_grow = false;
    }
}