use crossterm::{
    cursor::{Hide, MoveTo},
    event::{poll, read, Event, KeyCode},
    execute, queue,
    style::{Color, Print, SetBackgroundColor, SetForegroundColor},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen},
};
use std::io::{stdout, Write};
use std::time::Duration;

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone)]
enum BoardPiece {
    Snake,
    SnakeHead,
    Food,
    Empty,
}

struct Snake {
    body: Vec<(u8, u8)>,
    direction: Direction,
}

struct Game {
    snake: Snake,
    dimensions: (u8, u8),
    food: (u8, u8),
}

impl Game {
    fn new(dimensions: (u8, u8)) -> Game {
        Game {
            snake: Snake {
                body: vec![(0, 0), (0, 1), (0, 2)],
                direction: Direction::Down,
            },
            dimensions,
            food: (dimensions.0 - 1, dimensions.1 - 1),
        }
    }

    fn tick(&mut self) {
        let head = self.snake.body.first().unwrap();
        let mut next_pos: (u8, u8) = next_position(*head, self.snake.direction, self.dimensions);

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

    fn change_direction(&mut self, direction: Direction) {
        self.snake.direction = direction;
    }

    fn board_to_lines(&self) -> Vec<String> {
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

fn main() -> std::io::Result<()> {
    let mut game = Game::new((32, 32));

    let mut stdout = stdout();
    enable_raw_mode()?;
    execute!(
        stdout,
        EnterAlternateScreen,
        SetBackgroundColor(Color::DarkGrey),
        SetForegroundColor(Color::White),
        Hide
    )?;

    loop {
        if poll(Duration::from_millis(150))? {
            match read()? {
                Event::Key(event) => {
                    match event.code {
                        KeyCode::Up | KeyCode::Char('k') | KeyCode::Char('w') => {
                            game.change_direction(Direction::Up)
                        }
                        KeyCode::Down | KeyCode::Char('j') | KeyCode::Char('s') => {
                            game.change_direction(Direction::Down)
                        }
                        KeyCode::Left | KeyCode::Char('h') | KeyCode::Char('a') => {
                            game.change_direction(Direction::Left)
                        }
                        KeyCode::Right | KeyCode::Char('l') | KeyCode::Char('d') => {
                            game.change_direction(Direction::Right)
                        }
                        KeyCode::Esc => break,
                        _ => {}
                    };
                }
                _ => {}
            }
        } else {
            queue!(stdout, Clear(ClearType::All))?;

            game.board_to_lines()
                .iter()
                .enumerate()
                .for_each(|(i, line)| queue!(stdout, MoveTo(0, i as u16), Print(line)).unwrap());

            stdout.flush()?;
            game.tick();
        }
    }

    disable_raw_mode()?;
    Ok(())
}
