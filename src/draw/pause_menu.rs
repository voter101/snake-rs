use crossterm::{
    cursor::MoveTo,
    queue,
    style::{Print, SetBackgroundColor, SetForegroundColor},
};
use std::io::Stdout;

use crate::{consts, window::WindowDimensions};

pub fn draw_pause_screen(window_dim: WindowDimensions, stdout: &mut Stdout) -> std::io::Result<()> {
    let content = menu_lines();
    let (rows, cols) = menu_dimensions(&content);

    if window_dim.0 < rows || window_dim.1 < cols {
        draw_minimal_board(stdout)?;
        return Ok(());
    }

    draw_full_board(&content, window_dim, stdout)?;

    Ok(())
}

fn draw_full_board(
    content: &Vec<String>,
    window_dim: WindowDimensions,
    stdout: &mut Stdout,
) -> std::io::Result<()> {
    let (rows, cols) = menu_dimensions(&content);
    let (starting_row, starting_col) = (
        (window_dim.0 - rows as u16) / 2,
        (window_dim.1 - cols as u16) / 2,
    );

    queue!(
        stdout,
        SetBackgroundColor(consts::BACKGROUND_COLOR),
        SetForegroundColor(consts::BACKGROUND_TEXT_COLOR)
    )?;

    content.iter().enumerate().for_each(|(i, line)| {
        queue!(
            stdout,
            MoveTo(starting_col, starting_row + i as u16),
            Print(line)
        )
        .unwrap()
    });

    Ok(())
}

fn menu_lines() -> Vec<String> {
    (r#"PAUSED

    <esc> - Resume game
    <q>   - Quit game"#)
        .lines()
        .map(|l| l.trim().to_string())
        .collect()
}

fn menu_dimensions(menu_lines: &Vec<String>) -> (u16, u16) {
    let menu_border_width: u16 = 1;
    let menu_inner_padding: u16 = 1;
    let menu_outer_padding: u16 = 1;

    let added_size = menu_border_width * 2 + menu_inner_padding * 2 + menu_outer_padding * 2;

    (
        menu_lines.len() as u16 + added_size,
        menu_lines.iter().map(|l| l.len()).max().unwrap() as u16 + added_size,
    )
}

fn draw_minimal_board(stdout: &mut Stdout) -> std::io::Result<()> {
    queue!(
        stdout,
        MoveTo(0, 0),
        SetBackgroundColor(consts::BACKGROUND_COLOR),
        SetForegroundColor(consts::BACKGROUND_TEXT_COLOR),
        Print("PAUSED")
    )?;
    Ok(())
}
