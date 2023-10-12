use crate::board::{Board, GameState, COL, ROW};
/// 0 for draw,
/// W1 - depth for WinP1
/// W2 + depth for WinP2
pub type Score = i32;

pub const W1: Score = 10_000;
pub const W2: Score = -W1;

pub const EQUAL: Score = DELTA;
pub const MAX: Score = W1;
pub const MIN: Score = W2;

const MAXMOVES: i32 = (COL * ROW) as i32;

const MINW1: Score = W1 - MAXMOVES;
const MINW2: Score = W2 + MAXMOVES;

const DELTA: Score = 1;

pub fn getscore(board: &Board) -> Score {
    match board.gamestate() {
        GameState::Open => {
            let s = board.scoreboard().total_score();
            match s {
                0 => DELTA, // score is 0 only if is a draw, if heiristic is 0 return DELTA
                _ => s,
            }
        }
        GameState::Draw => 0,
        GameState::WinP1 => W1 - board.nmoves() as i32,
        GameState::WinP2 => W2 + board.nmoves() as i32,
    }
}

pub trait ShowScore {
    fn gamestate(&self) -> GameState;
}

impl ShowScore for Score {
    fn gamestate(&self) -> GameState {
        if *self == 0 {
            GameState::Draw
        } else if *self <= MINW2 {
            GameState::WinP2
        } else if *self >= MINW1 {
            GameState::WinP1
        } else {
            GameState::Open
        }
    }
}
