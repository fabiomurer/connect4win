use crate::board::{Board, GameState, COL, ROW};
/// 0 for draw,
/// W1 - depth for WinP1
/// W2 + depth for WinP2
pub type Score = i32;

pub const W1: Score = 10_000;
pub const W2: Score = -W1;

pub const EQUAL: Score = 0;
pub const MAX: Score = W1;
pub const MIN: Score = W2;

const MAXMOVES: i32 = (COL * ROW) as i32;

const MINW1: Score = W1 - MAXMOVES;
const MINW2: Score = W2 + MAXMOVES;

pub const DRAW: Score = MINW2 + 1;
const MINDRAW: Score = DRAW + MAXMOVES;

pub trait ScoreMethods {
    fn to_string(&self) -> String;
    fn gamestate(&self) -> GameState;
}

impl ScoreMethods for Score {
    fn to_string(&self) -> String {
        match self.gamestate() {
            GameState::Open => std::string::ToString::to_string(&self),
            GameState::Draw => {
                let d = (self - W1).abs();
                format!("Draw with {} stones", d)
            }
            GameState::WinP1 => {
                let d = (self - W1).abs();
                format!("WinP1 with {} stones", d)
            }
            GameState::WinP2 => {
                let d = (self - W2).abs();
                format!("WinP2 with {} stones", d)
            }
        }
    }

    fn gamestate(&self) -> GameState {
        if *self <= MINW2 {
            GameState::WinP2
        } else if *self <= MINDRAW {
            GameState::Draw
        } else if *self >= MINW1 {
            GameState::WinP1
        } else {
            GameState::Open
        }
    }
}

pub fn getscore(board: &Board) -> Score {
    match board.gamestate() {
        GameState::Open => board.scoreboard().total_score(),
        GameState::Draw => DRAW + board.nmoves() as i32,
        GameState::WinP1 => W1 - board.nmoves() as i32,
        GameState::WinP2 => W2 + board.nmoves() as i32,
    }
}
