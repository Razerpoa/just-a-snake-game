use crate::game::{Coordinates, Direction, Game};
use pathfinding::prelude::astar;
use std::cmp::Ordering;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32);

impl Pos {
    fn to_coordinates(&self) -> Coordinates {
        Coordinates { x: self.0, y: self.1 }
    }

    fn distance(&self, other: &Pos) -> u32 {
        (self.0.abs_diff(other.0) + self.1.abs_diff(other.1)) as u32
    }

    fn successors(&self, game: &Game) -> Vec<(Pos, u32)> {
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

            if !game.is_collision(&new_pos.to_coordinates()) {
                successors.push((new_pos, 1));
            }
        }
        successors
    }
}

pub fn find_path(game: &Game) -> Option<Direction> {
    let start = Pos(game.state.snake.get_head().x, game.state.snake.get_head().y);
    let goal = Pos(game.state.food.position.x, game.state.food.position.y);

    let result = astar(
        &start,
        |p| p.successors(game),
        |p| p.distance(&goal),
        |p| *p == goal,
    );

    if let Some((path, _)) = result {
        if path.len() > 1 {
            let next_move = &path[1];
            let head = game.state.snake.get_head();
            match (next_move.0.cmp(&head.x), next_move.1.cmp(&head.y)) {
                (Ordering::Less, _) => Some(Direction::Left),
                (Ordering::Greater, _) => Some(Direction::Right),
                (_, Ordering::Less) => Some(Direction::Up),
                (_, Ordering::Greater) => Some(Direction::Down),
                _ => None,
            }
        } else {
            None
        }
    } else {
        None
    }
}


