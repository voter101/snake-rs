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

type StyledBoard<'a> = Vec<Vec<StyledContent<&'a str>>>;

pub fn style_game_board(game: &Game) -> StyledBoard {
    let board_pieces = game.board_pieces();
    let inner_board = board_pieces
        .iter()
        .map(|line| {
            line.iter()
                .map(|field| match field {
                    BoardPiece::Snake => "O"
                        .with(consts::BOARD_FIELD_TEXT_COLOR)
                        .on(consts::BOARD_FIELD_BACKGROUND_COLOR)
                        .bold()
                        .dim(),
                    BoardPiece::SnakeHead => "#"
                        .with(consts::BOARD_FIELD_TEXT_COLOR)
                        .on(consts::BOARD_FIELD_BACKGROUND_COLOR),
                    BoardPiece::Food => "@"
                        .with(consts::BOARD_FIELD_TEXT_COLOR)
                        .on(consts::BOARD_FIELD_BACKGROUND_COLOR)
                        .bold(),
                    BoardPiece::Fruit => "$"
                        .with(consts::BOARD_FIELD_TEXT_COLOR)
                        .on(consts::BOARD_FIELD_BACKGROUND_COLOR)
                        .bold(),
                    BoardPiece::Empty => " "
                        .with(consts::BOARD_FIELD_TEXT_COLOR)
                        .on(consts::BOARD_FIELD_BACKGROUND_COLOR),
                })
                .collect::<Vec<_>>()
        })
        .collect();
    decorate_with_walls(inner_board)
}

fn decorate_with_walls(board: StyledBoard) -> StyledBoard {
    let mut result: StyledBoard = vec![];

    let wall_element = " ".on(consts::BOARD_BORDER_COLOR);
    let wall_row = vec![vec![wall_element.clone(); board.first().unwrap().len() + 2]];

    result.extend(wall_row.clone());
    result.extend(
        board
            .iter()
            .map(|line| {
                let mut res = vec![wall_element];
                res.extend(line);
                res.extend(vec![wall_element]);
                res
            })
            .collect::<Vec<_>>(),
    );
    result.extend(wall_row.clone());

    result
}
