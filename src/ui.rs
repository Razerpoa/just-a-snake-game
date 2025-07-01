use ratatui::{
    layout::{Alignment, Constraint, Direction as LayoutDirection, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
    Frame,
};
use crate::game::{Coordinates, Direction, GameState};

pub fn ui(f: &mut Frame, game_state: &GameState) {
    let chunks = Layout::default()
        .direction(LayoutDirection::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(10), Constraint::Percentage(90)].as_ref())
        .split(f.size());

    let score_text = format!("Score: {}", game_state.score);
    let speed_text = format!("Speed: {}", game_state.speed);
    let score_paragraph = Paragraph::new(format!("{} | {}", score_text, speed_text))
        .style(Style::default().add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center);
    f.render_widget(score_paragraph, chunks[0]);

    let game_area = chunks[1];
    let mut rows = vec![];
    for y in 0..game_state.height {
        let mut row = vec![];
        for x in 0..game_state.width {
            let coord = Coordinates { x, y };
            let mut cell = Cell::from(" ");
            if let Some(index) = game_state.snake.body.iter().position(|&c| c == coord) {
                let body_len = game_state.snake.body.len();
                let character = if index == 0 {
                    // Head
                    match game_state.snake.direction {
                        Direction::Up => "▲",
                        Direction::Down => "▼",
                        Direction::Left => "◄",
                        Direction::Right => "►",
                    }
                } else if index == body_len - 1 {
                    // Tail
                    let before_tail = game_state.snake.body.iter().nth(body_len - 2).unwrap();
                    if before_tail.y < coord.y { "╵" }
                    else if before_tail.y > coord.y { "╷" }
                    else if before_tail.x < coord.x { "╴" }
                    else { "╶" }
                } else {
                    // Body
                    let prev = game_state.snake.body.iter().nth(index - 1).unwrap();
                    let next = game_state.snake.body.iter().nth(index + 1).unwrap();
                    if prev.x == next.x { "│" }
                    else if prev.y == next.y { "─" }
                    else { // Corner
                        if (prev.y < coord.y && next.x > coord.x) || (prev.x > coord.x && next.y < coord.y) { "└" }
                        else if (prev.y < coord.y && next.x < coord.x) || (prev.x < coord.x && next.y < coord.y) { "┘" }
                        else if (prev.y > coord.y && next.x > coord.x) || (prev.x > coord.x && next.y > coord.y) { "┌" }
                        else { "┐" }
                    }
                };
                cell = Cell::from(character).style(Style::default().fg(Color::Green));
            } else if game_state.food.position == coord {
                cell = Cell::from("●").style(Style::default().fg(Color::LightRed));
            }
            row.push(cell);
        }
        rows.push(Row::new(row));
    }

    let widths = vec![Constraint::Length(1); game_state.width as usize];
    let table = Table::new(rows, widths)
        .block(Block::default().title("Snake").borders(Borders::ALL).style(Style::default().bg(Color::Black)));
    f.render_widget(table, game_area);

    if game_state.game_over {
        let game_over_text = "Game Over! Press 'q' to quit.";
        let game_over_block = Block::default()
            .title(game_over_text)
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Red).bg(Color::Black)); // Added background color
        let area = centered_rect(60, 20, f.size());
        f.render_widget(game_over_block, area);
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(LayoutDirection::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(LayoutDirection::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}
