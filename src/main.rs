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
    next_direction: Option<Direction>,
}

struct Game {
    snake: Snake,
    dimensions: (u8, u8),
    food: (u8, u8),
    score: u32,
    just_ate: bool,
}

impl Game {
    fn new(dimensions: (u8, u8)) -> Game {
        Game {
            snake: Snake::new(vec![(0, 0), (0, 1), (0, 2)], Direction::Down),
            dimensions,
            food: (dimensions.0 - 1, dimensions.1 - 1),
            score: 0,
            just_ate: false,
        }
    }

    fn tick(&mut self) {
        let direction = self.snake.next_direction();
        let head = self.snake.body.first().unwrap();
        let mut next_pos: (u8, u8) = next_position(*head, direction, self.dimensions);

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

        if next_pos == self.food {
            self.food = (0, 0);
            self.just_ate = true;
        }
    }

    fn change_direction(&mut self, direction: Direction) {
        self.snake.change_direction(direction)
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

impl Snake {
    fn new(body: Vec<(u8, u8)>, direction: Direction) -> Snake {
        Snake {
            body,
            direction,
            next_direction: None,
        }
    }

    fn change_direction(&mut self, direction: Direction) {
        match direction {
            Direction::Up | Direction::Down => match self.direction {
                Direction::Left | Direction::Right => self.next_direction = Some(direction),
                _ => {}
            },
            Direction::Left | Direction::Right => match self.direction {
                Direction::Up | Direction::Down => self.next_direction = Some(direction),
                _ => {}
            },
        }
    }

    fn next_direction(&mut self) -> Direction {
        match self.next_direction {
            Some(direction) => {
                self.direction = direction;
                self.next_direction = None;
            }
            None => {}
        }

        self.direction
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
