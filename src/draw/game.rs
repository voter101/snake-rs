use crossterm::{
    cursor::MoveTo,
    queue,
    style::{Print, PrintStyledContent, SetBackgroundColor, SetForegroundColor},
    terminal::{Clear, ClearType},
};
use std::{
    io::{Stdout, Write},
    time::Duration,
};

use crate::{
    board::style_game_board, config::Config, consts, game::Game, window::WindowDimensions,
};

pub fn is_window_big_enough(game: &Game, window_dim: WindowDimensions) -> bool {
    window_dim.0 >= game.dimensions.0 + 4 && window_dim.1 >= game.dimensions.1 + 4
}

pub fn draw_game_frame(
    game: &Game,
    config: &Config,
    window_dim: WindowDimensions,
    last_delta: Duration,
    stdout: &mut Stdout,
) -> std::io::Result<()> {
    queue_draw_board(game, window_dim, stdout)?;
    queue_draw_score(game, window_dim, stdout)?;
    queue_draw_fruit_timer(game, window_dim, stdout)?;
    if config.show_fps_counter {
        queue_draw_fps(last_delta, stdout)?;
    }

    stdout.flush()?;

    Ok(())
}

fn queue_draw_board(
    game: &Game,
    window_dim: WindowDimensions,
    stdout: &mut Stdout,
) -> std::io::Result<()> {
    let game_screen_start =
        game_screen_starting_position((window_dim.0, window_dim.1), game.dimensions);

    let board = style_game_board(game);

    board.iter().enumerate().for_each(|(row, line)| {
        line.iter().enumerate().for_each(|(col, element)| {
            queue!(
                stdout,
                MoveTo(
                    game_screen_start.1 + col as u16,
                    game_screen_start.0 + row as u16
                ),
                PrintStyledContent(*element)
            )
            .unwrap()
        });
    });

    Ok(())
}

fn queue_draw_score(
    game: &Game,
    window_dim: WindowDimensions,
    stdout: &mut Stdout,
) -> std::io::Result<()> {
    let (starting_row, starting_col) =
        game_screen_starting_position((window_dim.0, window_dim.1), game.dimensions);

    queue!(
        stdout,
        SetBackgroundColor(consts::BACKGROUND_COLOR),
        SetForegroundColor(consts::BACKGROUND_TEXT_COLOR)
    )?;

    let score_line = format!("Score: {}", game.score);
    queue!(
        stdout,
        MoveTo(starting_col, starting_row + (game.dimensions.0 as u16) + 2),
        Print(score_line)
    )?;

    Ok(())
}

fn queue_draw_fruit_timer(
    game: &Game,
    window_dim: WindowDimensions,
    stdout: &mut Stdout,
) -> std::io::Result<()> {
    let (starting_row, starting_col) =
        game_screen_starting_position((window_dim.0, window_dim.1), game.dimensions);

    // If fruit is not available, clear the line (this could be improved, we need to do it max once per fruit cycle)
    queue!(
        stdout,
        MoveTo(starting_col, starting_row - 1),
        Clear(ClearType::UntilNewLine),
    )?;

    if let Some(_) = game.fruit {
        let text_line = format!("$ {}", game.fruit.unwrap().1);

        queue!(
            stdout,
            SetBackgroundColor(consts::BACKGROUND_COLOR),
            SetForegroundColor(consts::BACKGROUND_TEXT_COLOR),
            Print(text_line)
        )?;
    }

    Ok(())
}

fn queue_draw_fps(last_delta: Duration, stdout: &mut Stdout) -> std::io::Result<()> {
    let delta = if last_delta.as_secs_f64() == 0. {
        1.0
    } else {
        last_delta.as_secs_f64()
    };
    let fps = (1.0 / delta) as u16;

    queue!(
        stdout,
        MoveTo(0, 0),
        SetBackgroundColor(consts::FPS_COUNTER_BACKGROUND_COLOR),
        SetForegroundColor(consts::FPS_COUNTER_TEXT_COLOR),
        Print(fps as u16),
        SetBackgroundColor(consts::BACKGROUND_COLOR),
    )?;

    Ok(())
}

fn game_screen_starting_position(
    window_dim: (u16, u16),
    board_dimensions: (u16, u16),
) -> (u16, u16) {
    (
        (window_dim.0 - (board_dimensions.0 + 2) as u16) / 2,
        (window_dim.1 - (board_dimensions.1 + 2) as u16) / 2,
    )
}
