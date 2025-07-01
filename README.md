# Rust Snake Game

A classic snake game implemented in Rust, running in the terminal.

## Features

*   **Classic Snake Gameplay:** Control the snake to eat food and grow longer.
*   **AI Player:** Watch an AI play the game using an A* pathfinding algorithm.
*   **Adjustable Speed:** Control the game's speed, making it as easy or as challenging as you like.
*   **Terminal-Based:** Runs directly in your terminal.

## Getting Started

### Prerequisites

*   [Rust](httpss://www.rust-lang.org/tools/install) programming language toolchain.

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

*   `--speed <SPEED>`: Set the initial speed of the game. Defaults to 20. A higher number means a faster game.

    ```bash
    cargo run --release -- --speed 30
    ```

### In-Game Controls

*   **Arrow Keys:** Move the snake (Up, Down, Left, Right).
*   **PageUp:** Increase the game speed.
*   **PageDown:** Decrease the game speed.
*   **q:** Quit the game.
