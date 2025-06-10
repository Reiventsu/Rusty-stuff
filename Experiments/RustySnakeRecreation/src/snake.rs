use std::collections::VecDeque;
use raylib::prelude::*;

//// Snake Direction control struct
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
    
}

//     std::deque<Vector2> body = {Vector2{6, 9}, Vector2{5, 9}, Vector2{4, 9}};
//     Direction direction = Direction::Right;
//     Direction nextDirection = Direction::Right;
//     bool shouldGrow = false;

//     void ProcessInput(const Direction newDirection) {
//         if (newDirection == inverse(direction)) return;
//         nextDirection = newDirection;
//     }

//     void UpdateSnake() {
//         direction = nextDirection;
//         const Vector2 movement = vector(direction);
//         const Vector2 newHead = Vector2Add(body.front(), movement);
//         body.push_front(newHead);
//         if (!shouldGrow) body.pop_back();
//         shouldGrow = false;
//     }

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