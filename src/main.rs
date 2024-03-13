use crossterm::{
    cursor::{Hide, MoveTo},
    event::{poll, read, Event, KeyCode},
    execute, queue,
    style::{Color, Print, SetBackgroundColor, SetForegroundColor},
    terminal::{
        disable_raw_mode, enable_raw_mode, window_size, Clear, ClearType, EnterAlternateScreen,
    },
};
use std::io::{stdout, Write};
use std::time::Duration;
use std::{collections::HashSet, io::Stdout};

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

    fn tick(&mut self) {
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

    fn change_direction(&mut self, direction: Direction) {
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

fn draw_ui(
    terminal_size: (u16, u16),
    game_dimensions: (u8, u8),
    stdout: &mut Stdout,
) -> std::io::Result<()> {
    let game_screen_start = game_screen_starting_position(terminal_size, game_dimensions);

    // Top and bottom line
    queue!(
        stdout,
        SetBackgroundColor(Color::Black),
        SetForegroundColor(Color::DarkGrey)
    )?;

    vec![
        // Left and right border
        (0..(game_dimensions.0 as u16))
            .flat_map(|row_i| {
                let row = game_screen_start.0 + row_i;
                let first_col = game_screen_start.1 - 1;
                let last_col = first_col + game_dimensions.1 as u16 + 1;

                vec![(row, first_col), (row, last_col)]
            })
            .collect::<Vec<(u16, u16)>>(),
        // Top and bottom border
        (0..(game_dimensions.1 as u16 + 2))
            .flat_map(|col_i| {
                let col = game_screen_start.1 + col_i - 1;
                let first_row = game_screen_start.0 - 1;
                let last_row = first_row + game_dimensions.0 as u16 + 1;

                vec![(first_row, col), (last_row, col)]
            })
            .collect::<Vec<(u16, u16)>>(),
    ]
    .iter()
    .flatten()
    .for_each(|(row, col)| queue!(stdout, MoveTo(*col, *row), Print(' ')).unwrap());

    Ok(())
}

fn game_screen_starting_position(
    terminal_size: (u16, u16),
    game_dimensions: (u8, u8),
) -> (u16, u16) {
    (
        (terminal_size.0 - game_dimensions.0 as u16) / 2,
        (terminal_size.1 - game_dimensions.1 as u16) / 2,
    )
}

fn main() -> std::io::Result<()> {
    let mut game = Game::new((8, 16));

    let mut stdout = stdout();
    enable_raw_mode()?;
    execute!(
        stdout,
        EnterAlternateScreen,
        SetBackgroundColor(Color::DarkGrey),
        SetForegroundColor(Color::White),
        Hide
    )?;

    execute!(stdout, Clear(ClearType::All))?;

    let terminal_size = window_size()?;
    draw_ui(
        (terminal_size.rows, terminal_size.columns),
        game.dimensions,
        &mut stdout,
    )?;

    loop {
        if poll(Duration::from_millis(250))? {
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
            let game_screen_start = game_screen_starting_position(
                (terminal_size.rows, terminal_size.columns),
                game.dimensions,
            );
            queue!(
                stdout,
                SetBackgroundColor(Color::DarkGreen),
                SetForegroundColor(Color::White)
            )?;
            game.board_to_lines()
                .iter()
                .enumerate()
                .for_each(|(i, line)| {
                    queue!(
                        stdout,
                        MoveTo(game_screen_start.1, i as u16 + game_screen_start.0),
                        Print(line)
                    )
                    .unwrap()
                });

            stdout.flush()?;
            game.tick();
        }
    }

    disable_raw_mode()?;
    Ok(())
}
