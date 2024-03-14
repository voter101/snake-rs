use std::collections::HashSet;

use crate::board::BoardPiece;
use crate::direction::Direction;
use crate::snake::Snake;

pub struct Game {
    snake: Snake,
    pub dimensions: (u8, u8),
    pub food: (u8, u8),
    pub score: u32,
    just_ate: bool,
}

impl Game {
    pub fn new(dimensions: (u8, u8)) -> Game {
        let mut new_obj = Game {
            snake: Snake::new(vec![(0, 0), (0, 1), (0, 2)], Direction::Down),
            dimensions,
            food: (0, 0),
            score: 0,
            just_ate: false,
        };
        new_obj.spawn_food();

        new_obj
    }

    fn game_over(&self) {
        println!("Game over!");
        println!("Final score: {}", self.score);
        panic!();
    }

    pub fn tick(&mut self) {
        let direction = self.snake.next_direction();
        let head = self.snake.body.first().unwrap();
        let mut next_pos: (u8, u8) = next_position(*head, direction, self.dimensions);
        let head_next = next_pos;

        if self.just_ate {
            self.snake.body = [vec![next_pos], self.snake.body.clone()]
                .iter()
                .flat_map(|e| e.clone())
                .collect::<Vec<(u8, u8)>>();
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
            self.game_over();
        }

        if head_next == self.food {
            self.score += 1;
            self.just_ate = true;
            self.spawn_food();
        }
    }

    pub fn change_direction(&mut self, direction: Direction) {
        self.snake.change_direction(direction)
    }

    fn spawn_food(&mut self) {
        let mut board_elements: HashSet<(u8, u8)> = (0..self.dimensions.0)
            .flat_map(|row| {
                (0..self.dimensions.1)
                    .map(|col| (row, col))
                    .collect::<Vec<_>>()
            })
            .collect();
        for snake_piece in &self.snake.body {
            board_elements.remove(&snake_piece);
        }

        match board_elements.iter().next() {
            Some(e) => self.food = e.clone(),
            None => self.game_over(),
        }
    }

    pub fn board_to_lines(&self) -> Vec<String> {
        let board = display_board(self);
        board
            .iter()
            .map(|line| {
                line.iter()
                    .map(|field| match field {
                        BoardPiece::Snake => 'O',
                        BoardPiece::SnakeHead => '#',
                        BoardPiece::Food => '@',
                        BoardPiece::Empty => ' ',
                    })
                    .collect::<String>()
            })
            .collect()
    }
}

fn next_position(pos: (u8, u8), direction: Direction, board_dimensions: (u8, u8)) -> (u8, u8) {
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

fn display_board(game: &Game) -> Vec<Vec<BoardPiece>> {
    let mut res =
        vec![vec![BoardPiece::Empty; game.dimensions.1 as usize]; game.dimensions.0 as usize];

    res[game.food.0 as usize][game.food.1 as usize] = BoardPiece::Food;
    for (i, snake_piece) in game.snake.body.iter().enumerate() {
        let piece: BoardPiece = if i == 0 {
            BoardPiece::SnakeHead
        } else {
            BoardPiece::Snake
        };
        res[snake_piece.0 as usize][snake_piece.1 as usize] = piece;
    }

    res
}
