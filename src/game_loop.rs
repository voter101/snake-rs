use crate::direction;
use crate::draw;
use crate::game::Game;
use crate::window;

use crossterm::event::{poll, read, Event, KeyCode};
use std::time::{Duration, Instant};

pub fn start_loop(game: &mut Game, stdout: &mut std::io::Stdout) -> std::io::Result<()> {
    draw::full_clear(stdout)?;
    draw::draw_ui(game.dimensions, window::window_dimensions(), stdout)?;

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
            draw::draw_game(&game, window_dim, stdout)?;
            draw::draw_fps(delta, window_dim, stdout)?;

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
                            draw::full_clear(stdout)?;
                            draw::draw_ui(game.dimensions, new_window_dim, stdout)?;
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(())
}
