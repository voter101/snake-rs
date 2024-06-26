use crate::config::Config;
use crate::consts::FPS_LIMIT;
use crate::direction;
use crate::draw;
use crate::draw::pause_menu::draw_pause_screen;
use crate::game::Game;
use crate::game::GameMode;
use crate::window;

use crossterm::event::{poll, read, Event, KeyCode};
use std::thread;
use std::time::{Duration, Instant};

pub enum GameLoopSignal {
    Exit,
    GameOver,
    Ok,
}

pub fn start_game(
    game: &mut Game,
    config: &Config,
    stdout: &mut std::io::Stdout,
) -> std::io::Result<GameLoopSignal> {
    draw::utils::full_clear(stdout)?;

    let mut last_frame_time = Instant::now();
    loop {
        let now = Instant::now();
        let delta = now - last_frame_time;

        // FPS limiter
        let frame_time = Duration::from_nanos(1000 * 1_000_000 / FPS_LIMIT);
        if delta < frame_time {
            thread::sleep(frame_time - delta);
            continue;
        }

        last_frame_time = now;

        let loop_res = match game.mode {
            GameMode::Game => loop_game_mode(game, config, delta, stdout),
            GameMode::Pause => loop_pause_mode(game, stdout),
        };

        match loop_res {
            Ok(signal) => match signal {
                GameLoopSignal::Ok => continue,
                _ => return Ok(signal),
            },
            Err(e) => {
                return Err(e);
            }
        };
    }
}

fn loop_game_mode(
    game: &mut Game,
    config: &Config,
    tick_delta: Duration,
    stdout: &mut std::io::Stdout,
) -> std::io::Result<GameLoopSignal> {
    let window_dim = window::window_dimensions();

    match game.tick(tick_delta) {
        Ok(_) => {}
        Err(_) => return Ok(GameLoopSignal::GameOver),
    };

    if draw::game::is_window_big_enough(&game, window_dim) {
        draw::game::draw_game_frame(&game, config, window_dim, tick_delta, stdout)?;

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
                        KeyCode::Esc => {
                            draw::utils::full_clear(stdout)?;
                            game.pause_game()
                        }
                        _ => {}
                    };
                }
                Event::Resize(cols, rows) => {
                    let new_window_dim = (rows, cols);
                    if draw::game::is_window_big_enough(&game, new_window_dim) {
                        draw::utils::full_clear(stdout)?;
                    }
                }
                _ => {}
            }
        }
    } else {
        game.pause_game();
        draw::utils::full_clear(stdout)?;
    }

    Ok(GameLoopSignal::Ok)
}

fn loop_pause_mode(
    game: &mut Game,
    stdout: &mut std::io::Stdout,
) -> std::io::Result<GameLoopSignal> {
    let window_dim = window::window_dimensions();

    draw_pause_screen(window_dim, stdout)?;

    if poll(Duration::from_millis(0))? {
        match read()? {
            Event::Key(event) => {
                match event.code {
                    KeyCode::Char('q') | KeyCode::Char('x') => return Ok(GameLoopSignal::Exit),

                    KeyCode::Esc => {
                        draw::utils::full_clear(stdout)?;
                        game.unpause_game()
                    }
                    _ => {}
                };
            }
            Event::Resize(cols, rows) => {
                let new_window_dim = (rows, cols);
                draw::utils::full_clear(stdout)?;
                draw_pause_screen(new_window_dim, stdout)?;
            }
            _ => {}
        }
    }

    Ok(GameLoopSignal::Ok)
}
