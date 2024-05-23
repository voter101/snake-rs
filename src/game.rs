use rand::Rng;
use std::collections::HashSet;
use std::time::Duration;

use crate::board::BoardPiece;
use crate::direction::Direction;
use crate::snake::Snake;
use crate::utils::manhattan_distance;

pub enum GameMode {
    Game,
    Pause,
}

pub struct Game {
    pub mode: GameMode,
    snake: Snake,
    pub dimensions: (u16, u16),
    pub food: (u16, u16),
    pub fruit: Option<((u16, u16), u16)>,
    pub score: u32,
    just_ate: bool,
    moves_until_next_fruit: u8,
    next_tick_in: Duration,
    speed: Duration,
    difficulty: u16,
}

impl Game {
    pub fn new(dimensions: (u16, u16), difficulty: u16) -> Game {
        let difficulty = difficulty.clamp(1, 9);
        let speed = Duration::from_millis(350 - difficulty as u64 * 30);
        let mut new_obj = Game {
            mode: GameMode::Game,
            snake: Snake::new(vec![(0, 0), (0, 1), (0, 2)], Direction::Down),
            dimensions,
            food: (0, 0),
            fruit: None,
            score: 0,
            just_ate: false,
            moves_until_next_fruit: 120,
            next_tick_in: speed,
            speed,
            difficulty,
        };
        new_obj.spawn_food().unwrap();

        new_obj
    }

    pub fn pause_game(&mut self) {
        self.mode = GameMode::Pause
    }

    pub fn unpause_game(&mut self) {
        self.mode = GameMode::Game
    }

    pub fn tick(&mut self, delta: Duration) -> Result<(), ()> {
        if !self.can_tick(delta) {
            self.next_tick_in -= delta;
            return Ok(());
        }

        // FIXME: This mechanism does not work well in case of very low FPS
        self.next_tick_in = self.speed;

        let direction = self.snake.next_direction();
        let head = self.snake.body.first().unwrap();
        let mut next_pos: (u16, u16) = next_position(*head, direction, self.dimensions);
        let head_next = next_pos;

        if self.just_ate {
            self.snake.body = [vec![next_pos], self.snake.body.clone()]
                .iter()
                .flat_map(|e| e.clone())
                .collect::<Vec<(u16, u16)>>();
            self.just_ate = false
        } else {
            self.snake.body = self
                .snake
                .body
                .iter()
                .map(|e| {
                    let tmp = next_pos;
                    next_pos = *e;
                    tmp
                })
                .collect::<_>();
        }

        if self.snake.body[1..].contains(&head_next) {
            return Err(());
        }

        if head_next == self.food {
            self.score += self.difficulty as u32;
            self.just_ate = true;
            match self.spawn_food() {
                Err(_) => return Err(()),
                _ => {}
            };
        }

        if let Some((fruit, remaining_moves)) = self.fruit {
            if remaining_moves == 0 {
                self.moves_until_next_fruit = rand::thread_rng().gen_range(30..180);
                self.fruit = None;
            } else if head_next == fruit {
                self.score += remaining_moves as u32 * self.difficulty as u32;
                self.just_ate = true;
                self.moves_until_next_fruit = rand::thread_rng().gen_range(30..180);
                self.fruit = None;
            } else {
                self.fruit = Some((fruit, remaining_moves - 1));
            }
        } else {
            if self.moves_until_next_fruit == 0 {
                self.spawn_fruit();
            } else {
                self.moves_until_next_fruit -= 1;
            }
        }
        Ok(())
    }

    fn can_tick(&self, delta: Duration) -> bool {
        self.next_tick_in < delta
    }

    pub fn change_direction(&mut self, direction: Direction) {
        self.snake.change_direction(direction)
    }

    fn spawn_food(&mut self) -> Result<(), ()> {
        let candidate = self.element_spawn_candidate();
        match candidate {
            Some(e) => self.food = e.clone(),
            None => return Err(()),
        }
        Ok(())
    }

    fn spawn_fruit(&mut self) {
        if let Some(candidate) = self.element_spawn_candidate() {
            let distance = manhattan_distance(candidate, *self.snake.body.first().unwrap());
            let allowed_moves = (distance * 2) as u16;

            self.fruit = Some((candidate, allowed_moves));
        }
    }

    fn element_spawn_candidate(&mut self) -> Option<(u16, u16)> {
        let mut board_elements: HashSet<(u16, u16)> = (0..self.dimensions.0)
            .flat_map(|row| {
                (0..self.dimensions.1)
                    .map(|col| (row, col))
                    .collect::<Vec<_>>()
            })
            .collect();

        for snake_piece in &self.snake.body {
            board_elements.remove(&snake_piece);
        }

        if let Some((fruit, _)) = self.fruit {
            board_elements.remove(&fruit);
        }

        board_elements.remove(&self.food);

        let snake_head = &self.snake.body.first().unwrap();

        // Sometime random is way too close to the head
        board_elements
            .iter()
            .take(3)
            .map(|e| e.clone())
            .max_by(|a, b| {
                manhattan_distance(*a, **snake_head).cmp(&manhattan_distance(*b, **snake_head))
            })
    }

    pub fn board_pieces(&self) -> Vec<Vec<BoardPiece>> {
        let mut res =
            vec![vec![BoardPiece::Empty; self.dimensions.1 as usize]; self.dimensions.0 as usize];

        res[self.food.0 as usize][self.food.1 as usize] = BoardPiece::Food;

        if let Some(((row, col), _)) = self.fruit {
            res[row as usize][col as usize] = BoardPiece::Fruit;
        }

        for (i, snake_piece) in self.snake.body.iter().enumerate() {
            let piece: BoardPiece = if i == 0 {
                BoardPiece::SnakeHead
            } else {
                BoardPiece::Snake
            };
            res[snake_piece.0 as usize][snake_piece.1 as usize] = piece;
        }

        res
    }
}

fn next_position(
    pos: (u16, u16),
    direction: Direction,
    board_dimensions: (u16, u16),
) -> (u16, u16) {
    match direction {
        Direction::Up => {
            if pos.0 == 0 {
                (board_dimensions.0 - 1, pos.1)
            } else {
                (pos.0 - 1, pos.1)
            }
        }
        Direction::Down => {
            if pos.0 == board_dimensions.0 - 1 {
                (0, pos.1)
            } else {
                (pos.0 + 1, pos.1)
            }
        }
        Direction::Left => {
            if pos.1 == 0 {
                (pos.0, board_dimensions.1 - 1)
            } else {
                (pos.0, pos.1 - 1)
            }
        }
        Direction::Right => {
            if pos.1 == board_dimensions.1 - 1 {
                (pos.0, 0)
            } else {
                (pos.0, pos.1 + 1)
            }
        }
    }
}
