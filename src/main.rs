pub mod game;
pub mod ui;
pub mod ai;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{io, time::{Duration, Instant}};
use clap::Parser;
use crate::game::Game;
use crate::ai::find_path;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The initial speed of the game
    #[arg(short, long, default_value_t = 5)]
    speed: u64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let size = terminal.size()?;
    let effective_width = (size.width as i32 - 4) / 2 - 2;
    let score_area_height = ((size.height as f32 - 2.0) * 0.1).floor() as i32;
    let effective_height = size.height as i32 - 2 - score_area_height - 2;

    let mut game = Game::new(effective_width, effective_height);
    game.state.speed = args.speed;
    let mut last_update = Instant::now();

    loop {
        terminal.draw(|f| ui::ui(f, &game.state))?;

        if crossterm::event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if game.state.game_over {
                    if let KeyCode::Char('q') = key.code {
                        break;
                    }
                } else {
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('+') => game.state.increase_speed(),
                        KeyCode::Char('-') => game.state.decrease_speed(),
                        _ => {}
                    }
                }
            }
        }

        if !game.state.game_over {
            if let Some(direction) = find_path(&game) {
                game.state.change_direction(direction);
            }

            if last_update.elapsed() >= Duration::from_millis(200 / game.state.speed) {
                game.state.update();
                last_update = Instant::now();
            }
        }
    }

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
