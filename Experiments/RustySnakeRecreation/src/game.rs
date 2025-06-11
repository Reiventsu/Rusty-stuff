// class SnakeGame {
// public:
//     Snake snake;
//     Food food;
//     bool gameRunning = true;
//     int gameScore = 0;

//     SnakeGame() : food(snake.body) {
//         snake.Reset();
//     }
//    

//     void CheckCollisionWithFood() {
//         if (Vector2Equals(snake.body.front(), food.position)) {
//             std::cout << "Eating Food" << std::endl;
//             food.position = Food::GenerateRandomPos(snake.body);
//             snake.shouldGrow = true;
//             gameScore++;
//         }
//     }

//     void Update() {
//         if (gameRunning) {
//             snake.UpdateSnake();
//             CheckCollisionWithFood();
//             CheckCollisionWithEdges();
//             CheckCollisionWithTail();
//         }
//     }

//     void Draw() const {
//         food.DrawFood();
//         snake.DrawSnake();
//     }

//     void CheckCollisionWithEdges() {
//         if (snake.body.front().x == static_cast<float>(cellCount) || snake.body.front().x == -1) {
//             GameOver();
//         }
//         if (snake.body.front().y == static_cast<float>(cellCount) || snake.body.front().y == -1) {
//             GameOver();
//         }
//     }

//     void CheckCollisionWithTail() {
//         if (ElementInDeque(snake.body.front(),
//                            std::next(snake.body.begin()), snake.body.end())) {
//             GameOver();
//         }
//     }

//     void GameOver() {
//         std::cout << "GAME OVER" << std::endl;
//         snake.Reset();
//         food.position = Food::GenerateRandomPos(snake.body);
//         gameRunning = false;
//     }
// };