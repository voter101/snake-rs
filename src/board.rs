use crossterm::style::{StyledContent, Stylize};

use crate::{consts, game::Game};

#[derive(Clone)]
pub enum BoardPiece {
    Snake,
    SnakeHead,
    Food,
    Fruit,
    Empty,
}

pub fn style_game_board(game: &Game) -> Vec<Vec<StyledContent<&str>>> {
    let board_pieces = game.board_pieces();
    board_pieces
        .iter()
        .map(|line| {
            line.iter()
                .map(|field| match field {
                    BoardPiece::Snake => "O"
                        .with(consts::BOARD_FIELD_TEXT_COLOR)
                        .on(consts::BOARD_FIELD_BACKGROUND_COLOR),
                    BoardPiece::SnakeHead => "#"
                        .with(consts::BOARD_FIELD_TEXT_COLOR)
                        .on(consts::BOARD_FIELD_BACKGROUND_COLOR)
                        .bold(),
                    BoardPiece::Food => "@"
                        .with(consts::BOARD_FIELD_TEXT_COLOR)
                        .on(consts::BOARD_FIELD_BACKGROUND_COLOR),
                    BoardPiece::Fruit => "$"
                        .with(consts::BOARD_FIELD_TEXT_COLOR)
                        .on(consts::BOARD_FIELD_BACKGROUND_COLOR),
                    BoardPiece::Empty => " "
                        .with(consts::BOARD_FIELD_TEXT_COLOR)
                        .on(consts::BOARD_FIELD_BACKGROUND_COLOR),
                })
                .collect::<Vec<_>>()
        })
        .collect()
}
