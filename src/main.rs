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
    match game_loop::start_game(&mut game, &mut stdout) {
        Ok(signal) => match signal {
            game_loop::GameLoopSignal::Ok => {
                terminal::unmount_from_terminal(&mut stdout).unwrap();
                println!("Unexpected error");
            }
            game_loop::GameLoopSignal::Exit => {
                terminal::unmount_from_terminal(&mut stdout).unwrap();
                println!("Thanks for playing! You scored {} points!", game.score);
            }
            game_loop::GameLoopSignal::GameOver => {
                terminal::unmount_from_terminal(&mut stdout).unwrap();
                println!("Game over! You scored {} points!", game.score);
            }
        },
        Err(_) => {
            terminal::unmount_from_terminal(&mut stdout).unwrap();
            println!("Unexpected error");
        }
    };
}
