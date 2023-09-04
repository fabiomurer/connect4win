use crate::board::*;
use crate::score::*;
use crate::transposition_table::*;
use crate::Move::*;
use crate::timer::*;


pub struct Engine {
    board: Board,
    timer: Timer,
    table: Table,
} 

impl Engine {
    pub fn new(seconds: u64, table_size: usize) -> Engine {
        Engine { 
            board: Board::new(), 
            timer: Timer::new(seconds), 
            table: Table::new(table_size),
        }
    }

    pub fn alpha_beta(&mut self, alpha: Score, beta: Score, depth: u8) -> Result<Score, TimeoutError> {

        let saved_score = self.table.get(&self.board.bitboard());
        match  saved_score {
            Some(score) => Ok(score),
            None => {
                // robe
            }
        }
    }
}