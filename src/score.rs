use crate::board::{GameState, COL, ROW};
/// 0 for draw,
/// W1 - depth for WinP1
/// W2 + depth for WinP2
pub type Score = i32;

pub const W1: Score = 1000;
pub const W2: Score = -W1;

pub const EQUAL: Score = 0;
pub const MAX: Score = W1;
pub const MIN: Score = W2;

const MAXMOVES: i32 = (COL * ROW) as i32;

const MINW1: Score = W1 - MAXMOVES;
const MINW2: Score = W2 + MAXMOVES;

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
