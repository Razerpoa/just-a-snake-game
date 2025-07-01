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

    // 1. Try to find a path to the food and check if it's safe.
    let food_pos = Pos(game.state.food.position.x, game.state.food.position.y);
    let result_to_food = astar(
        &start,
        |p| p.successors(game, false),
        |p| p.distance(&food_pos),
        |p| *p == food_pos,
    );

    let mut food_path_is_safe = false;
    if let Some((_path, _)) = &result_to_food {
        game.state.path_to_food_found = true;

        let mut future_game = game.clone();
        let mut future_snake_body = game.state.snake.body.clone();
        future_snake_body.push_front(game.state.food.position);
        future_game.state.snake.body = future_snake_body;

        let future_head_pos = future_game.state.snake.get_head();
        let future_tail_pos = future_game.state.snake.get_tail();
        let future_start = Pos(future_head_pos.x, future_head_pos.y);
        let future_goal = Pos(future_tail_pos.x, future_tail_pos.y);

        let escape_path = astar(
            &future_start,
            |p| p.successors(&future_game, true),
            |p| p.distance(&future_goal),
            |p| *p == future_goal,
        );

        if escape_path.is_some() {
            game.state.path_to_tail_after_eat_found = true;
            food_path_is_safe = true;
        }
    }

    // 2. Try to find a path to the tail.
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

    let mut tail_path_found = false;
    if let Some((_path, _)) = &result_to_tail {
        game.state.path_to_tail_found = true;
        tail_path_found = true;
    }

    // Decision logic based on calculated paths and safety.

    // Strategy 1: Go to food if safe.
    if game.state.path_to_food_found && food_path_is_safe {
        if let Some((path, _)) = result_to_food {
            if path.len() > 1 {
                if let Some(direction) = get_direction(&head, &path[1].to_coordinates()) {
                    game.state.ai_strategy = "Food".to_string();
                    return Some(direction);
                }
            }
        }
    }

    // Strategy 2: Fill available space if eating food is not safe but tail is reachable.
    // This is the user's requested condition.
    if game.state.path_to_food_found && !food_path_is_safe && tail_path_found {
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
            game.state.ai_strategy = "Space-Fill (Conditional)".to_string();
            return Some(*direction);
        }
    }

    // Strategy 3: Go to tail if a path to tail exists (and not handled by conditional space-fill).
    if tail_path_found {
        if let Some((path, _)) = result_to_tail {
            if path.len() > 1 {
                if let Some(direction) = get_direction(&head, &path[1].to_coordinates()) {
                    game.state.ai_strategy = "Tail".to_string();
                    return Some(direction);
                }
            }
        }
    }

    // Strategy 4: Fallback to general space-fill (if no other strategy applies).
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
        game.state.ai_strategy = "Space-Fill (Fallback)".to_string();
        return Some(*direction);
    } else {
        // Strategy 5: Trapped.
        game.state.is_trapped = true;
        game.state.ai_strategy = "Trapped Fallback".to_string();
        return Some(game.state.snake.direction);
    }
}
