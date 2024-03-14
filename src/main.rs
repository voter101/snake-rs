use crossterm::{
    cursor::Hide,
    event::{poll, read, Event, KeyCode},
    execute,
    style::{Color, SetBackgroundColor, SetForegroundColor},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen},
};
use std::io::stdout;
use std::time::Duration;

mod board;
mod direction;
mod draw;
mod game;
mod snake;

fn main() -> std::io::Result<()> {
    let mut game = game::Game::new((8, 16));

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

    draw::draw_ui(game.dimensions, &mut stdout)?;

    loop {
        if poll(Duration::from_millis(250))? {
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
                _ => {}
            }
        } else {
            draw::draw_game(&game, &mut stdout)?;
            game.tick();
        }
    }

    disable_raw_mode()?;
    Ok(())
}
