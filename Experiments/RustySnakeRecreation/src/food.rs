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