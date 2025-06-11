// Class for food for the snake to eat
// class Food {
// public:
//     Vector2 position{};
//     Texture2D texture{};

//     explicit Food(const std::deque<Vector2> &snakeBody) {
//         const Image image = LoadImage("../Graphics/FoodImage.png");
//         texture = LoadTextureFromImage(image);
//         UnloadImage(image);
//         position = GenerateRandomPos(snakeBody);
//     }

//     ~Food() {
//         UnloadTexture(texture);
//     }

//     void DrawFood() const {
//         DrawTexture(
//             texture,
//             offset + static_cast<int>(position.x) * cellSize,
//             offset + static_cast<int>(position.y) * cellSize,
//             WHITE
//         );
//     }

//     static Vector2 GenerateRandomCell() {
//         const int x = GetRandomValue(0, cellCount - 1);
//         const int y = GetRandomValue(0, cellCount - 1);
//         return Vector2{
//             static_cast<float>(x),
//             static_cast<float>(y)
//         };
//     }
pub fn generate_random_cell(rl: &mut RaylibHandle) -> Vector2 {
    let rand_x:i32 = rl.get_random_value(0..CELL_COUNT);
    let rand_y:i32 = rl.get_random_value(0..CELL_COUNT);
    Vector2::new(rand_x as f32, rand_y as f32) 
}

//     static Vector2 GenerateRandomPos(const std::deque<Vector2> &snakeBody) {
//         int attempts = 0;

//         Vector2 position = GenerateRandomCell();
//         while (ElementInDeque(position, snakeBody.begin(), snakeBody.end())) {
//             if (constexpr int maxAttempts = 1000; ++attempts > maxAttempts) {
//                 std::cerr << "Failed to generate food position! " << std::endl;
//                 break;
//             }
//             position = GenerateRandomCell();
//         }
//         return position;
//     }
// };

use raylib::core::texture::Image;
use raylib::prelude::*;
use crate::CELL_COUNT;

pub struct Food {
    position: Vector2,
    texture: Texture2D,
}

//explicit Food(const std::deque<Vector2> &snakeBody) {
//         const Image image = LoadImage("../Graphics/FoodImage.png");
//         texture = LoadTextureFromImage(image);
//         UnloadImage(image);
//         position = GenerateRandomPos(snakeBody);
//     }

impl Food {
    pub fn food_image(&mut self, d: &mut RaylibDrawHandle, thread: &RaylibThread) {
        let food = Image::load_image("Graphics/FoodImage.png").unwrap();
        self.texture = d.load_texture_from_image(thread, &food)
            .expect("Buy (ANY) gpu >:(");
        let position = 
    }
}
