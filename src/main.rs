pub mod game;
pub mod graphics;
pub mod ai;

use macroquad::prelude::*;
use clap::Parser;
use crate::game::Game;
use crate::ai::find_path;
use crate::graphics::draw_game;
use std::time::{Duration, Instant};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The initial speed of the game
    #[arg(short, long, default_value_t = 5)]
    speed: u64,
    /// The size of each cell in pixels
    #[arg(long, default_value_t = 20.0)]
    cell_size: f32,
}

#[macroquad::main("Snake AI")]
async fn main() {
    let args = Args::parse();

    let mut game = Game::new(screen_width() as i32 / args.cell_size as i32, screen_height() as i32 / args.cell_size as i32);
    game.state.speed = args.speed;
    game.state.cell_size = args.cell_size;

    let mut last_update = Instant::now();

    loop {
        if is_key_pressed(KeyCode::Q) {
            break;
        }

        if game.state.game_over {
            if is_key_pressed(KeyCode::Space) {
                game = Game::new(screen_width() as i32 / args.cell_size as i32, screen_height() as i32 / args.cell_size as i32);
                game.state.speed = args.speed; // Retain previous speed
                game.state.cell_size = args.cell_size;
            }
        } else {
            if is_key_down(KeyCode::PageUp) {
                game.state.increase_speed();
            }
            if is_key_down(KeyCode::PageDown) {
                game.state.decrease_speed();
            }

            let time_per_frame = Duration::from_millis(200 / game.state.speed);
            if last_update.elapsed() >= time_per_frame {
                if let Some(direction) = find_path(&mut game) {
                    game.state.change_direction(direction);
                }
                game.state.update();
                last_update = Instant::now();
            }
        }

        clear_background(BLACK);
        draw_game(&game.state);

        next_frame().await
    }
}