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
    body: VecDeque<Vector2>,
    direction: Direction,
    next_direction: Direction,
    should_grow: bool,
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


    //     void ProcessInput(const Direction newDirection) {
    //         if (newDirection == inverse(direction)) return;
    //         nextDirection = newDirection;
    //     }
    pub fn process_input(&mut self, new_direction: Direction) {
        if new_direction != self.direction.inverse_direction() {
            self.next_direction = new_direction;
        }
    }


    //     void UpdateSnake() {
    //         direction = nextDirection;
    //         const Vector2 movement = vector(direction);
    //         const Vector2 newHead = Vector2Add(body.front(), movement);
    //         body.push_front(newHead);
    //         if (!shouldGrow) body.pop_back();
    //         shouldGrow = false;
    //     }
    pub fn update(&mut self) {
        self.direction = self.next_direction;
        let movement = self.direction.to_vector();
        let new_head = self.body.front().unwrap().add(movement);
        self.body.push_front(new_head);
        
        if !self.should_grow {
            self.body.pop_back();
        }
        self.should_grow = true;
    }
    
    pub fn draw(&self) {}
}
//     void DrawSnake() const {
//         for (const auto &[x, y]: body) {
//             const auto segment = Rectangle{
//                 static_cast<float>(offset) + x * static_cast<float>(cellSize),
//                 static_cast<float>(offset) + y * static_cast<float>(cellSize),
//                 static_cast<float>(cellSize),
//                 static_cast<float>(cellSize)
//             };
//             DrawRectangleRounded(segment, 0.5, 6, darkGreen);
//         }
//     }

//     void Reset() {
//         body = {Vector2{6, 9}, Vector2{5, 9}, Vector2{4, 9}};
//         direction = Direction::Right;
//         nextDirection = Direction::Right;
//     }
// };
