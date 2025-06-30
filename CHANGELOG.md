# Changelog

## Snake Game

This is the initial version of the snake game, built from the ground up.

### Added

*   **Core Gameplay:**
    *   Implemented the basic snake game logic, including snake movement, food generation, and collision detection.
    *   The snake grows longer when it eats food.
    *   The game ends if the snake hits a wall or itself.
*   **Terminal User Interface (TUI):**
    *   Created a TUI using `ratatui` to render the game in the terminal.
    *   The game board is dynamically sized to fit the terminal window.
    *   The UI displays the current score, which updates in real-time.
    *   A "Game Over" message is shown when the game ends.
*   **Visuals & UX:**
    *   The snake is rendered with a distinct arrow head and a continuous body.
    *   The food is displayed as a large, visible circle.
    *   The score is displayed in a bold, centered font.
    *   The snake's movement is responsive, with immediate feedback on key presses.
*   **Code Structure:**
    *   The codebase is organized into `game`, `ui`, and `main` modules to improve scalability and maintainability.

### Fixed

*   Prevented the snake from dying when the user quickly presses keys for opposite directions.
*   The game board now correctly resizes with the terminal, preventing the snake from dying unexpectedly at the edges of the screen.
