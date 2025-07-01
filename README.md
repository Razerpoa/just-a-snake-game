# Rust Snake AI Game

A classic snake game implemented in Rust, featuring an AI player.

## Features

*   **Classic Snake Gameplay:** The AI controls the snake to eat food and grow longer.
*   **AI Player:** An AI plays the game using an A* pathfinding algorithm.
*   **Adjustable Speed:** Control the game's speed, making it as easy or as challenging as you like.
*   **Graphical Interface:** Runs with a graphical window using Macroquad.

## Getting Started

### Prerequisites

*   [Rust](https://www.rust-lang.org/tools/install) programming language toolchain.

### Building

To build the game, clone the repository and use Cargo:

```bash
git clone <repository-url>
cd rust-snake-game
cargo build --release
```

### Running the Game

You can run the game using `cargo run`:

```bash
cargo run --release
```

## Usage

### Command-Line Options

*   `--speed <SPEED>`: Set the initial speed of the game. Defaults to 5. A higher number means a faster game.

    ```bash
    cargo run --release -- --speed 10
    ```
*   `--cell-size <SIZE>`: Set the size of each cell in pixels. Defaults to 20.0.

    ```bash
    cargo run --release -- --cell-size 30.0
    ```

### In-Game Controls

*   **PageUp:** Increase the game speed.
*   **PageDown:** Decrease the game speed.
*   **Space:** Restart the game after game over.
*   **Q:** Quit the game.
