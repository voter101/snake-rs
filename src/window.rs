use crate::game::Game;
use crossterm::terminal::window_size;

pub fn is_window_big_enough(game: &Game) -> bool {
    let terminal_size = window_size().unwrap();

    terminal_size.rows >= game.dimensions.0 + 4 && terminal_size.columns >= game.dimensions.1 + 4
}
