use crate::game::{Coordinates, Direction, Game};
use pathfinding::prelude::astar;
use std::cmp::Ordering;
use std::collections::{HashSet, VecDeque};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32);

impl Pos {
    fn to_coordinates(&self) -> Coordinates {
        Coordinates { x: self.0, y: self.1 }
    }

    fn distance(&self, other: &Pos) -> u32 {
        (self.0.abs_diff(other.0) + self.1.abs_diff(other.1)) as u32
    }

    fn successors(&self, game: &Game, ignore_tail: bool) -> Vec<(Pos, u32)> {
        let mut successors = Vec::new();
        for dir in [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ] {
            let new_pos = match dir {
                Direction::Up => Pos(self.0, self.1 - 1),
                Direction::Down => Pos(self.0, self.1 + 1),
                Direction::Left => Pos(self.0 - 1, self.1),
                Direction::Right => Pos(self.0 + 1, self.1),
            };

            if !game.is_collision(&new_pos.to_coordinates(), ignore_tail) {
                successors.push((new_pos, 1));
            }
        }
        successors
    }
}

fn get_direction(from: &Coordinates, to: &Coordinates) -> Option<Direction> {
    match (to.x.cmp(&from.x), to.y.cmp(&from.y)) {
        (Ordering::Less, _) => Some(Direction::Left),
        (Ordering::Greater, _) => Some(Direction::Right),
        (_, Ordering::Less) => Some(Direction::Up),
        (_, Ordering::Greater) => Some(Direction::Down),
        _ => None,
    }
}

fn get_next_pos(direction: &Direction, head: &Coordinates) -> Coordinates {
    match direction {
        Direction::Up => Coordinates { x: head.x, y: head.y - 1 },
        Direction::Down => Coordinates { x: head.x, y: head.y + 1 },
        Direction::Left => Coordinates { x: head.x - 1, y: head.y },
        Direction::Right => Coordinates { x: head.x + 1, y: head.y },
    }
}

fn count_reachable_space(start_pos: &Coordinates, game: &Game) -> usize {
    let mut reachable_space = HashSet::new();
    let mut queue = VecDeque::new();

    if !game.is_collision(start_pos, true) {
        queue.push_back(*start_pos);
        reachable_space.insert(*start_pos);
    }

    while let Some(pos) = queue.pop_front() {
        for dir in [Direction::Up, Direction::Down, Direction::Left, Direction::Right] {
            let next_pos = match dir {
                Direction::Up => Coordinates { x: pos.x, y: pos.y - 1 },
                Direction::Down => Coordinates { x: pos.x, y: pos.y + 1 },
                Direction::Left => Coordinates { x: pos.x - 1, y: pos.y },
                Direction::Right => Coordinates { x: pos.x + 1, y: pos.y },
            };

            if !game.is_collision(&next_pos, true) && !reachable_space.contains(&next_pos) {
                reachable_space.insert(next_pos);
                queue.push_back(next_pos);
            }
        }
    }

    reachable_space.len()
}

pub fn find_path(game: &mut Game) -> Option<Direction> {
    let head = game.state.snake.get_head();
    let start = Pos(head.x, head.y);

    // Reset debugging flags
    game.state.path_to_food_found = false;
    game.state.path_to_tail_found = false;
    game.state.path_to_tail_after_eat_found = false;
    game.state.is_trapped = false;
    game.state.ai_strategy = "Fallback".to_string(); // Default to fallback
    

    // 1. Try to find a path to the food.
    let food_pos = Pos(game.state.food.position.x, game.state.food.position.y);
    let result_to_food = astar(
        &start,
        |p| p.successors(game, false),
        |p| p.distance(&food_pos),
        |p| *p == food_pos,
    );

    if let Some((path, _)) = result_to_food {
        game.state.path_to_food_found = true;
        
        // Simulate the game state after eating the food to check for traps.
        let mut future_game = game.clone();
        let mut future_snake_body = game.state.snake.body.clone();
        future_snake_body.push_front(game.state.food.position); // New head is the food position.
        // The tail doesn't get removed because it just ate.
        future_game.state.snake.body = future_snake_body;

        // Check for an escape path in this future state.
        let future_head_pos = future_game.state.snake.get_head();
        let future_tail_pos = future_game.state.snake.get_tail();
        let future_start = Pos(future_head_pos.x, future_head_pos.y);
        let future_goal = Pos(future_tail_pos.x, future_tail_pos.y);

        let escape_path = astar(
            &future_start,
            |p| p.successors(&future_game, true), // Allow path to go over the tail.
            |p| p.distance(&future_goal),
            |p| *p == future_goal,
        );

        if escape_path.is_some() {
            game.state.path_to_tail_after_eat_found = true;
            
            // The path is safe. Return the first step.
            if path.len() > 1 {
                if let Some(direction) = get_direction(&head, &path[1].to_coordinates()) {
                    game.state.ai_strategy = "Food".to_string();
                    return Some(direction);
                }
            }
        }
        
        // If no escape path, it's a trap. Fall through to survival logic.
    }

    // 2. If no safe path to food, try to find a path to the tail.
    let tail_pos = {
        let tail = game.state.snake.get_tail();
        Pos(tail.x, tail.y)
    };
    let result_to_tail = astar(
        &start,
        |p| p.successors(game, true),
        |p| p.distance(&tail_pos),
        |p| *p == tail_pos,
    );

    if let Some((path, _)) = result_to_tail {
        game.state.path_to_tail_found = true;
        
        if path.len() > 1 {
            if let Some(direction) = get_direction(&head, &path[1].to_coordinates()) {
                game.state.ai_strategy = "Tail".to_string();
                return Some(direction);
            }
        }
    }

    // 3. If no path to tail, find the move that leads to the most open space.
    
    let best_move = [
        Direction::Right,
        Direction::Left,
        Direction::Up,
        Direction::Down,
    ]
    .iter()
    .filter(|d| !game.is_collision(&get_next_pos(d, &head), false))
    .max_by_key(|d| {
        let next_pos = get_next_pos(d, &head);
        let mut temp_game = game.clone();
        temp_game.state.snake.body.push_front(next_pos);
        if !temp_game.state.snake.digesting {
            temp_game.state.snake.body.pop_back();
        }
        count_reachable_space(&next_pos, &temp_game)
    });

    if let Some(direction) = best_move {
        game.state.ai_strategy = "Space-Fill".to_string();
        
        Some(*direction)
    } else {
        // 4. If no move is possible, it's trapped.
        game.state.is_trapped = true;
        game.state.ai_strategy = "Trapped Fallback".to_string();
        Some(game.state.snake.direction)
    }
}
