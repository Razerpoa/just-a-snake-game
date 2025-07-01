use macroquad::prelude::*;
use crate::game::GameState;

pub fn draw_game(game_state: &GameState) {
    // Draw game area border
    draw_rectangle_lines(0.0, 0.0, screen_width(), screen_height(), 5.0, GRAY);

    // Draw food
    draw_rectangle(
        game_state.food.position.x as f32 * game_state.cell_size,
        game_state.food.position.y as f32 * game_state.cell_size,
        game_state.cell_size,
        game_state.cell_size,
        RED,
    );

    // Draw snake
    for (i, segment) in game_state.snake.body.iter().enumerate() {
        let color = if i == 0 {
            GREEN // Head
        } else {
            LIME // Body
        };
        draw_rectangle(
            segment.x as f32 * game_state.cell_size,
            segment.y as f32 * game_state.cell_size,
            game_state.cell_size,
            game_state.cell_size,
            color,
        );
    }

    // Draw score and speed
    draw_text(
        &format!("Score: {}", game_state.score),
        10.0,
        screen_height() - 30.0,
        30.0,
        WHITE,
    );
    draw_text(
        &format!("Speed: {}", game_state.speed),
        150.0,
        screen_height() - 30.0,
        30.0,
        WHITE,
    );

    // Draw debug info (top left)
    let debug_text_top_left = format!(
        "AI: {} | Food: ({},{}) | Len: {}",
        game_state.ai_strategy,
        game_state.food.position.x,
        game_state.food.position.y,
        game_state.snake.body.len()
    );
    draw_text(
        &debug_text_top_left,
        10.0,
        30.0,
        20.0,
        Color::new(0.0, 1.0, 1.0, 1.0),
    );

    // Draw debug flags (vertical on right side)
    let debug_flags = vec![
        format!("Path to Food: {}", game_state.path_to_food_found),
        format!("Path to Tail: {}", game_state.path_to_tail_found),
        format!("Path to Tail After Eat: {}", game_state.path_to_tail_after_eat_found),
        format!("Is Trapped: {}", game_state.is_trapped),
    ];

    let mut y_offset = 80.0; // Adjusted starting Y-offset to avoid overlap
    for (i, flag_text) in debug_flags.iter().enumerate() {
        let color = match i {
            0 => if game_state.path_to_food_found { GREEN } else { RED },
            1 => if game_state.path_to_tail_found { GREEN } else { RED },
            2 => if game_state.path_to_tail_after_eat_found { GREEN } else { RED },
            3 => if game_state.is_trapped { RED } else { GREEN }, // Trapped is bad, so red when true
            _ => WHITE, // Fallback for any unexpected flags
        };
        draw_text(
            flag_text,
            10.0, // Moved to the left side
            y_offset,
            20.0,
            color,
        );
        y_offset += 25.0; // Adjust spacing as needed
    }

    if game_state.game_over {
        let game_over_text = "Game Over! Press 'Q' to quit or Space to continue.";
        let text_dimensions = measure_text(game_over_text, None, 40, 1.0);
        draw_text(
            game_over_text,
            screen_width() / 2.0 - text_dimensions.width / 2.0,
            screen_height() / 2.0 - text_dimensions.height / 2.0,
            40.0,
            RED,
        );
    }
}