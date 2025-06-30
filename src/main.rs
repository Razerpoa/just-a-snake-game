pub mod game;
pub mod ui;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{io, time::{Duration, Instant}};
use crate::game::GameState;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let size = terminal.size()?;
    let width = size.width as i32 / 2;
    let height = size.height as i32 - 4;

    let mut game_state = GameState::new(width, height);
    let mut last_update = Instant::now();

    loop {
        terminal.draw(|f| ui::ui(f, &game_state))?;

        if crossterm::event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                let changed = match key.code {
                    KeyCode::Char('q') => {
                        break;
                    }
                    KeyCode::Left => game_state.change_direction(game::Direction::Left),
                    KeyCode::Right => game_state.change_direction(game::Direction::Right),
                    KeyCode::Up => game_state.change_direction(game::Direction::Up),
                    KeyCode::Down => game_state.change_direction(game::Direction::Down),
                    _ => false,
                };
                if changed {
                    game_state.update();
                    last_update = Instant::now();
                }
            }
        }

        if !game_state.game_over && last_update.elapsed() >= Duration::from_millis(200) {
            game_state.update();
            last_update = Instant::now();
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
