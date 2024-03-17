use crate::consts;
use crossterm::{
    cursor::MoveTo,
    queue,
    style::{Color, Print, SetBackgroundColor, SetForegroundColor},
    terminal::{window_size, Clear, ClearType},
};
use std::{
    io::{Stdout, Write},
    time::Duration,
};

use crate::game::Game;

pub fn draw_ui(game_dimensions: (u8, u8), stdout: &mut Stdout) -> std::io::Result<()> {
    let terminal_size = window_size()?;
    let game_screen_start =
        game_screen_starting_position((terminal_size.rows, terminal_size.columns), game_dimensions);

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

pub fn draw_game(game: &Game, stdout: &mut Stdout) -> std::io::Result<()> {
    let terminal_size = window_size()?;
    let game_screen_start =
        game_screen_starting_position((terminal_size.rows, terminal_size.columns), game.dimensions);

    queue_draw_score(game, stdout)?;

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

    Ok(())
}

fn queue_draw_score(game: &Game, stdout: &mut Stdout) -> std::io::Result<()> {
    let terminal_size = window_size()?;
    let game_screen_start =
        game_screen_starting_position((terminal_size.rows, terminal_size.columns), game.dimensions);

    queue!(
        stdout,
        SetBackgroundColor(Color::DarkGrey),
        SetForegroundColor(Color::White),
    )?;

    let score_line = format!("Score: {}", game.score);
    queue!(
        stdout,
        MoveTo(
            game_screen_start.1,
            game_screen_start.0 + (game.dimensions.0 as u16) + 1
        ),
        Print(score_line)
    )?;

    Ok(())
}

pub fn draw_fps(last_delta: Duration, stdout: &mut Stdout) -> std::io::Result<()> {
    let terminal_size = window_size()?;
    let delta = if last_delta.as_secs_f64() == 0. {
        1.0
    } else {
        last_delta.as_secs_f64()
    };
    let fps = (1.0 / delta) as u16;

    // I know I could use log10, but I was lazy
    let fps_length = fps.to_string().len() as u16;

    queue!(
        stdout,
        MoveTo(terminal_size.columns - fps_length, 0),
        SetBackgroundColor(consts::BACKGROUND_COLOR),
        // In case FPS count changes the decimal length
        Clear(ClearType::CurrentLine),
        SetBackgroundColor(Color::DarkGreen),
        SetForegroundColor(Color::Black),
        Print(fps as u16)
    )?;

    Ok(())
}
