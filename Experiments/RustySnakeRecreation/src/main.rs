pub mod food;
pub mod game;
pub mod snake;

use raylib::prelude::*;

// Global values

mod const_colors {
    use raylib::color::*;

    // [C++]constexpr auto green = Color(173, 204, 96, 255);
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

// constexpr int cellSize = 30;
// constexpr int cellCount = 25;
// constexpr int offset = 75;

const CELL_SIZE: i32 = 30;
const CELL_COUNT: i32 = 25;
const OFFSET: i32 = 75;

// bool ElementInDeque(const Vector2 element, const auto &start, const auto &end) {
//     return std::ranges::any_of(start, end, [&element](const Vector2 &i) {
//         return Vector2Equals(i, element);
//     });
// }

fn element_in_deque() {}

// main function of the program makes the window and handles input that interact with the other parts of the program.
// int main() {
//     std::cout << "Starting the game..." << std::endl;
//     InitWindow(2 * offset + cellSize * cellCount, 2 * offset + cellSize * cellCount, "Snake Game");
//     SetTargetFPS(60);
//     float accumulatedTime = 0.0f;

//     auto game = SnakeGame();

//     while (!WindowShouldClose()) {
//         BeginDrawing();

//         if (IsKeyPressed(KEY_UP)) game.snake.ProcessInput(Snake::Direction::Up);
//         if (IsKeyPressed(KEY_DOWN)) game.snake.ProcessInput(Snake::Direction::Down);
//         if (IsKeyPressed(KEY_LEFT)) game.snake.ProcessInput(Snake::Direction::Left);
//         if (IsKeyPressed(KEY_RIGHT)) game.snake.ProcessInput(Snake::Direction::Right);

//         if (!game.gameRunning && IsKeyPressed(KEY_SPACE)) {
//             game.gameRunning = true;
//             game.snake.Reset();
//             game.gameScore = 0;
//         }

//         constexpr float moveInterval = 0.2f;
//         const float deltaTime = GetFrameTime();
//         accumulatedTime += deltaTime;

//         const float speedMultiplier = IsKeyDown(KEY_SPACE) ? 2.0f : 1.0f;

//         if (const float effectiveInterval = moveInterval / speedMultiplier; accumulatedTime >= effectiveInterval) {
//             game.Update();
//             accumulatedTime -= effectiveInterval;
//         }

//         ClearBackground(green);
//         DrawRectangleLinesEx(Rectangle{
//                                  static_cast<float>(offset) - 5,
//                                  static_cast<float>(offset) - 5,
//                                  static_cast<float>(cellSize * cellCount + 10),
//                                  static_cast<float>(cellSize * cellCount + 10)
//                              },
//                              5, darkGreen);

//         DrawText(TextFormat("%i", game.gameScore), offset - 5, offset + cellSize * cellCount + 10, 40, darkGreen);
//         game.Draw();

//         if (!game.gameRunning) {
//             DrawRectangle(0, 0, GetScreenWidth(), GetScreenHeight(), Fade(BLACK, 0.6f));

//             DrawText("GAME OVER!",
//                      offset + cellSize * cellCount / 2 - MeasureText("GAME OVER!", 70) / 2,
//                      offset + cellSize * cellCount / 2 - 120,
//                      70, green);

//             const char *scoreText = TextFormat("your score was: %i", game.gameScore);
//             DrawText(scoreText,
//                      offset + cellSize * cellCount / 2 - MeasureText(scoreText, 40) / 2,
//                      offset + cellSize * cellCount / 2 - 40,
//                      40, green);

//             DrawText("Press SPACE to restart or ESC to close",
//                      offset + cellSize * cellCount / 2 - MeasureText("Press SPACE to Restart or ESC to close", 30) / 2,
//                      offset + cellSize * cellCount / 2 + 20,
//                      30, green);
//         }
//         EndDrawing();
//     }
//     CloseWindow();
//     return 0;
// }

/*
use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Hello, World")
        .build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
    }
}
InitWindow(2 * offset + cellSize * cellCount, 2 * offset + cellSize * cellCount, "Snake Game");

 DrawRectangleLinesEx(Rectangle{
//                                  static_cast<float>(offset) - 5,
//                                  static_cast<float>(offset) - 5,
//                                  static_cast<float>(cellSize * cellCount + 10),
//                                  static_cast<float>(cellSize * cellCount + 10)
//                              },
//                              5, darkGreen);
 */
fn main() {
    let (mut rl, thread) = raylib::init()
        .size(
            2 * OFFSET + CELL_SIZE * CELL_COUNT,
            2 * OFFSET + CELL_SIZE * CELL_COUNT,
        )
        .title("RustySnake Recreation")
        .build();

    while !rl.window_should_close() {
        let mut game_window = rl.begin_drawing(&thread);

        game_window.clear_background(const_colors::green());

        game_window.draw_rectangle_lines_ex(
            rrect(
                OFFSET - 5,
                OFFSET - 5,
                CELL_SIZE * CELL_COUNT + 10,
                CELL_SIZE * CELL_COUNT + 10,
            ),
            5.0,
            const_colors::dark_green(),
        )
    }
}
