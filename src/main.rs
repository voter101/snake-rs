use std::io::stdout;

mod board;
mod consts;
mod direction;
mod draw;
mod game;
mod game_loop;
mod snake;
mod terminal;
mod utils;
mod window;

fn main() {
    let difficulty = 9;
    let mut game = game::Game::new((8, 16), difficulty);
    let mut stdout = stdout();

    terminal::hook_into_terminal(&mut stdout).unwrap();
    game_loop::start_loop(&mut game, &mut stdout).unwrap();
    terminal::unmount_from_terminal().unwrap();
}
