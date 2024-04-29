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

enum LoopSignal {
    Exit,
    Ok,
}

pub fn start_game(game: &mut Game, stdout: &mut std::io::Stdout) -> std::io::Result<()> {
    draw::full_clear(stdout)?;
    draw::draw_ui(game.dimensions, window::window_dimensions(), stdout)?;

    let mut last_frame_time = Instant::now();
    loop {
        // We get window size once per tick to avoid terminal changing size in
        // the middle of the frame render
        let now = Instant::now();
        let delta = now - last_frame_time;

        // FPS limiter
        let frame_time = Duration::from_nanos(1000 * 1_000_000 / FPS_LIMIT);
        if delta < frame_time {
            thread::sleep(frame_time - delta);
            continue;
        }

        last_frame_time = now;

        match game.mode {
            GameMode::Game => loop_game_mode(game, delta, stdout)?,
            GameMode::Pause => {
                match loop_pause_mode(game, stdout) {
                    Ok(signal) => match signal {
                        LoopSignal::Exit => break,
                        LoopSignal::Ok => continue,
                    },
                    Err(e) => {
                        return Err(e);
                    }
                };
            }
        };
    }
    Ok(())
}

fn loop_game_mode(
    game: &mut Game,
    tick_delta: Duration,
    stdout: &mut std::io::Stdout,
) -> std::io::Result<LoopSignal> {
    let window_dim = window::window_dimensions();

    game.tick(tick_delta);

    if draw::is_window_big_enough(&game, window_dim) {
        draw::draw_game(&game, window_dim, stdout)?;
        draw::draw_fps(tick_delta, window_dim, stdout)?;

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
                            draw::full_clear(stdout)?;
                            game.pause_game()
                        }
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
    } else {
        game.pause_game();
        draw::full_clear(stdout)?;
    }

    Ok(LoopSignal::Ok)
}

fn loop_pause_mode(game: &mut Game, stdout: &mut std::io::Stdout) -> std::io::Result<LoopSignal> {
    let window_dim = window::window_dimensions();

    draw_pause_screen(window_dim, stdout)?;

    if poll(Duration::from_millis(0))? {
        match read()? {
            Event::Key(event) => {
                match event.code {
                    KeyCode::Char('q') | KeyCode::Char('x') => return Ok(LoopSignal::Exit),

                    KeyCode::Esc => {
                        draw::full_clear(stdout)?;
                        draw::draw_ui(game.dimensions, window_dim, stdout)?;
                        game.unpause_game()
                    }
                    _ => {}
                };
            }
            Event::Resize(cols, rows) => {
                let new_window_dim = (rows, cols);
                draw::full_clear(stdout)?;
                draw_pause_screen(new_window_dim, stdout)?;
            }
            _ => {}
        }
    }

    Ok(LoopSignal::Ok)
}
