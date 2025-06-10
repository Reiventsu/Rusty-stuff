// Le snek
// class Snake {
// public:
//     enum class Direction {
//         Right, // 0 + 2 % 4 == 2 (Left)
//         Up,    // 1 + 2 % 4 == 3 (Down)
//         Left,  // 2 + 2 % 4 == 0 (Right)
//         Down,  // 3 + 2 % 4 == 1 (Up)
//     };

//     static constexpr Direction inverse(Direction d) noexcept {
//         return static_cast<Direction>((static_cast<int>(d) + 2) % 4);
//     }

//     static constexpr Vector2 vector(Direction d) noexcept {
//         constexpr Vector2 v[] {
//             {+1,  0}, // Right
//             { 0, -1}, // Up
//             {-1,  0}, // Left
//             { 0, +1} // Down
//         };
//         return v[static_cast<int>(d)];
//     }

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