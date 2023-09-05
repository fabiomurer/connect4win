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

    fn move_sort(&mut self) -> Vec<u8> {
        let v = self.board.legal_moves();
        let mut mv: Vec<Move> = Vec::new();
        let mut out: Vec<u8> = Vec::new();

        for m in v {
            self.board.make_move(m);
            mv.push(Move::new(m, self.board.player(), self.board.evaluate(), 1));

            self.board.unmake_move();
        }
        mv.sort();
        if self.board.player() == Player::P1 {
            mv.reverse();
        }

        for m in mv {
            out.push(m.col());
        }
        out
    }

    pub fn alpha_beta(&mut self, mut alpha: Score, mut beta: Score, depth: u8) -> Result<Score, TimeoutError> {
        let saved_score: Option<Score>;
        if depth >= 2 {
            saved_score = self.table.get(&self.board.bitboard());
        } else {
            saved_score = None;
        }

        match  saved_score {
            Some(score) => Ok(score),
            None => {
                let moves: Vec<u8>;
                let mut eval: Score;

                if depth >= 1 {
                    moves = self.move_sort();
                } else {
                    moves = self.board.legal_moves();
                }

                if depth <= 0 || self.board.gamestate() != GameState::OPEN {
                    return Ok(self.board.evaluate());
                } else {
                    match self.board.player() {
                        Player::P1 => {
                            eval = MIN;
                            for m in moves {
                                match self.timer.check() {
                                    Ok(_) => {
                                        self.board.make_move(m);
                                        eval = eval.max(self.board.evaluate());
                                        alpha = alpha.max(eval);
                                        self.board.unmake_move();

                                        if alpha > beta {
                                            break;
                                        }
                                    },
                                    Err(TimeoutError) => {
                                        return Err(TimeoutError);
                                    }
                                }
                            }
                        },
                        Player::P2 => {
                            eval = MAX;
                            for m in moves {
                                match self.timer.check() {
                                    Ok(_) => {
                                        self.board.make_move(m);
                                        eval = eval.min(self.board.evaluate());
                                        beta = beta.min(eval);
                                        self.board.unmake_move();

                                        if alpha > beta {
                                            break;
                                        }
                                    },
                                    Err(TimeoutError) => {
                                        return Err(TimeoutError);
                                    }
                                }
                            }
                        }
                    }
                }
                if depth >= 2 {
                    self.table.set(self.board.bitboard(), eval);
                }
                return Ok(eval);
            }
        }
    }
}