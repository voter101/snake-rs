use crossterm::{
    cursor::Hide,
    event::{poll, read, Event, KeyCode},
    execute,
    style::{SetBackgroundColor, SetForegroundColor},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen},
};
use std::io::stdout;
use std::time::{Duration, Instant};

mod board;
mod consts;
mod direction;
mod draw;
mod game;
mod snake;
mod utils;
mod window;

fn main() -> std::io::Result<()> {
    let difficulty = 9;
    let mut game = game::Game::new((8, 16), difficulty);

    let mut stdout = stdout();
    enable_raw_mode()?;
    execute!(
        stdout,
        EnterAlternateScreen,
        SetBackgroundColor(consts::BACKGROUND_COLOR),
        SetForegroundColor(consts::BACKGROUND_TEXT_COLOR),
        Hide
    )?;

    execute!(stdout, Clear(ClearType::All))?;

    draw::draw_ui(game.dimensions, window::window_dimensions(), &mut stdout)?;

    let mut last_frame_time = Instant::now();

    loop {
        // We get window size once per tick to avoid terminal changing size in
        // the middle of the frame render
        let window_dim = window::window_dimensions();
        let now = Instant::now();
        let delta = now - last_frame_time;
        last_frame_time = now;

        game.tick(delta);
        if draw::is_window_big_enough(&game, window_dim) {
            draw::draw_game(&game, window_dim, &mut stdout)?;
            draw::draw_fps(delta, window_dim, &mut stdout)?;

            if poll(Duration::from_millis(0))? {
                match read()? {
                    Event::Key(event) => {
                        match event.code {
                            KeyCode::Up | KeyCode::Char('k') | KeyCode::Char('w') => {
                                game.change_direction(direction::Direction::Up)
                            }
                            KeyCode::Down | KeyCode::Char('j') | KeyCode::Char('s') => {
                                game.change_direction(direction::Direction::Down)
                            }
                            KeyCode::Left | KeyCode::Char('h') | KeyCode::Char('a') => {
                                game.change_direction(direction::Direction::Left)
                            }
                            KeyCode::Right | KeyCode::Char('l') | KeyCode::Char('d') => {
                                game.change_direction(direction::Direction::Right)
                            }
                            KeyCode::Esc => break,
                            _ => {}
                        };
                    }
                    Event::Resize(cols, rows) => {
                        let new_window_dim = (rows, cols);
                        if draw::is_window_big_enough(&game, new_window_dim) {
                            draw::full_clear(&mut stdout)?;
                            draw::draw_ui(game.dimensions, new_window_dim, &mut stdout)?;
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    disable_raw_mode()?;
    Ok(())
}
