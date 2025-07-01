use std::collections::LinkedList;
use rand::Rng;
use std::time::{Duration, Instant};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Coordinates {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone)]
pub struct Snake {
    pub body: LinkedList<Coordinates>,
    pub direction: Direction,
    pub digesting: bool,
}

impl Snake {
    pub fn new(start: Coordinates, length: u32, direction: Direction) -> Self {
        let mut body = LinkedList::new();
        for i in 0..length {
            body.push_back(Coordinates {
                x: start.x - i as i32,
                y: start.y,
            });
        }
        Self { body, direction, digesting: false }
    }

    pub fn get_tail(&self) -> Coordinates {
        *self.body.back().unwrap()
    }

    pub fn get_head(&self) -> Coordinates {
        *self.body.front().unwrap()
    }

    pub fn move_forward(&mut self) {
        let head = self.body.front().unwrap();
        let new_head = match self.direction {
            Direction::Up => Coordinates { x: head.x, y: head.y - 1 },
            Direction::Down => Coordinates { x: head.x, y: head.y + 1 },
            Direction::Left => Coordinates { x: head.x - 1, y: head.y },
            Direction::Right => Coordinates { x: head.x + 1, y: head.y },
        };
        self.body.push_front(new_head);

        if !self.digesting {
            self.body.pop_back();
        } else {
            self.digesting = false;
        }
    }

    pub fn check_collision(&self, width: i32, height: i32) -> bool {
        let head = self.body.front().unwrap();
        if head.x < 0 || head.x >= width || head.y < 0 || head.y >= height {
            return true; // Wall collision
        }
        for (i, segment) in self.body.iter().enumerate() {
            if i > 0 && *segment == *head {
                return true; // Self collision
            }
        }
        false
    }
}

#[derive(Clone)]
pub struct Food {
    pub position: Coordinates,
}

impl Food {
    pub fn new(width: i32, height: i32, snake_body: &LinkedList<Coordinates>) -> Self {
        let mut rng = rand::thread_rng();
        let mut position = Coordinates {
            x: rng.gen_range(2..width - 2),
            y: rng.gen_range(2..height - 2),
        };
        while snake_body.contains(&position) {
            position = Coordinates {
                x: rng.gen_range(2..width - 2),
                y: rng.gen_range(2..height - 2),
            };
        }
        Self { position }
    }
}

pub struct Game {
    pub state: GameState,
}

impl Clone for Game {
    fn clone(&self) -> Self {
        Game {
            state: self.state.clone(),
        }
    }
}

impl Game {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            state: GameState::new(width, height),
        }
    }

    pub fn is_collision(&self, pos: &Coordinates, ignore_tail: bool) -> bool {
        if pos.x < 0 || pos.x >= self.state.width || pos.y < 0 || pos.y >= self.state.height {
            return true; // Wall collision
        }

        let body_iter = if ignore_tail {
            self.state.snake.body.iter().take(self.state.snake.body.len().saturating_sub(1))
        } else {
            self.state.snake.body.iter().take(self.state.snake.body.len())
        };

        for segment in body_iter {
            if *segment == *pos {
                return true; // Self collision
            }
        }
        false
    }
}

pub struct GameState {
    pub snake: Snake,
    pub food: Food,
    pub score: u32,
    pub game_over: bool,
    pub width: i32,
    pub height: i32,
    pub last_direction: Option<Direction>,
    pub speed: u64,
    pub cell_size: f32,
    pub path_to_food_found: bool,
    pub path_to_tail_found: bool,
    pub path_to_tail_after_eat_found: bool,
    pub is_trapped: bool,
    pub ai_strategy: String,
    pub speed_change_timer: Instant,
}

impl Clone for GameState {
    fn clone(&self) -> Self {
        Self {
            snake: self.snake.clone(),
            food: self.food.clone(),
            score: self.score,
            game_over: self.game_over,
            width: self.width,
            height: self.height,
            last_direction: self.last_direction,
            speed: self.speed,
            cell_size: self.cell_size,
            path_to_food_found: self.path_to_food_found,
            path_to_tail_found: self.path_to_tail_found,
            path_to_tail_after_eat_found: self.path_to_tail_after_eat_found,
            is_trapped: self.is_trapped,
            ai_strategy: self.ai_strategy.clone(),
            speed_change_timer: self.speed_change_timer,
        }
    }
}

impl GameState {
    pub fn new(width: i32, height: i32) -> Self {
        let snake_start = Coordinates { x: width / 2, y: height / 2 };
        let snake = Snake::new(snake_start, 3, Direction::Right);
        let food = Food::new(width, height, &snake.body);
        Self {
            snake,
            food,
            score: 0,
            game_over: false,
            width,
            height,
            last_direction: None,
            speed: 5,
            cell_size: 20.0,
            path_to_food_found: false,
            path_to_tail_found: false,
            path_to_tail_after_eat_found: false,
            is_trapped: false,
            ai_strategy: "None".to_string(),
            speed_change_timer: Instant::now(),
        }
    }

    pub fn increase_speed(&mut self) {
        if self.speed_change_timer.elapsed() >= Duration::from_millis(100) {
            self.speed = (self.speed + 1).min(1000);
            self.speed_change_timer = Instant::now();
        }
    }

    pub fn decrease_speed(&mut self) {
        if self.speed_change_timer.elapsed() >= Duration::from_millis(100) {
            self.speed = (self.speed - 1).max(1);
            self.speed_change_timer = Instant::now();
        }
    }


    pub fn update(&mut self) {
        if self.game_over {
            return;
        }

        if let Some(direction) = self.last_direction.take() {
            self.snake.direction = direction;
        }

        self.snake.move_forward();

        if self.snake.check_collision(self.width, self.height) {
            self.game_over = true;
            return;
        }

        let head = self.snake.body.front().unwrap();
        if *head == self.food.position {
            self.score += 1;
            self.snake.digesting = true;
            self.food = Food::new(self.width, self.height, &self.snake.body);
        }
    }

    pub fn change_direction(&mut self, direction: Direction) -> bool {
        let current_direction = &self.snake.direction;
        let can_change = match (current_direction, &direction) {
            (Direction::Up, Direction::Down) | (Direction::Down, Direction::Up) |
            (Direction::Left, Direction::Right) | (Direction::Right, Direction::Left) => false,
            _ => true,
        };

        if can_change {
            self.last_direction = Some(direction);
        }
        can_change
    }
}
