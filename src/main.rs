use clap::Parser;

use std::io::stdout;

mod board;
mod config;
mod consts;
mod direction;
mod draw;
mod game;
mod game_loop;
mod snake;
mod terminal;
mod utils;
mod window;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Game speed
    #[arg(short, long, default_value_t = 5, value_parser = clap::value_parser!(u16).range(1..=9))]
    difficulty: u16,

    /// Board width
    #[arg(long, default_value_t = 16, value_parser = clap::value_parser!(u16).range(3..256))]
    width: u16,

    /// Board height
    #[arg(long, default_value_t = 8, value_parser = clap::value_parser!(u16).range(3..256))]
    height: u16,

    /// Show FPS counter
    #[arg(long, default_value_t = false)]
    show_fps: bool,
}

fn main() {
    let args = Args::parse();

    let config = config::Config {
        show_fps_counter: args.show_fps,
    };
    let difficulty = args.difficulty;
    let mut game = game::Game::new((args.height, args.width), difficulty);

    let mut stdout = stdout();

    terminal::hook_into_terminal(&mut stdout).unwrap();

    match game_loop::start_game(&mut game, &config, &mut stdout) {
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
