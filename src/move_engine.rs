use crate::board::*;
use crate::score::*;
use crate::transposition_table::*;
use crate::r#move::*;
use crate::timer::*;


pub struct Engine {
    timer: Timer,
    table: Table,
} 

impl Engine {
    pub fn new(seconds: u64, table_size: usize) -> Engine {
        Engine {
            timer: Timer::new(seconds), 
            table: Table::new(table_size),
        }
    }

    fn move_sort(board: &mut Board) -> Vec<u8> {
        let v = board.legal_moves();
        let mut mv: Vec<Move> = Vec::new();
        let mut out: Vec<u8> = Vec::new();

        for m in v {
            board.make_move(m);
            mv.push(Move::new(m, board.player(), board.evaluate(), 1));

            board.unmake_move();
        }
        mv.sort();
        if board.player() == Player::P1 {
            mv.reverse();
        }

        for m in mv {
            out.push(m.col());
        }
        out
    }

    pub fn alpha_beta(&mut self, board: &mut Board, mut alpha: Score, mut beta: Score, depth: u8) -> Result<Score, TimeoutError> {
        let saved_score: Option<Score>;
        if depth >= 2 {
            saved_score = self.table.get(&board.bitboard());
        } else {
            saved_score = None;
        }

        match  saved_score {
            Some(score) => Ok(score),
            None => {
                let moves: Vec<u8>;
                let mut eval: Score;

                if depth >= 1 {
                    moves = Self::move_sort(board);
                } else {
                    moves = board.legal_moves();
                }

                if depth <= 0 || board.gamestate() != GameState::OPEN {
                    return Ok(board.evaluate());
                } else {
                    match board.player() {
                        Player::P1 => {
                            eval = MIN;
                            for m in moves {
                                match self.timer.check() {
                                    Ok(_) => {
                                        board.make_move(m);
                                        match self.alpha_beta(board, alpha, beta, depth - 1) {
                                            Ok(newscore) => {
                                                eval = eval.max(newscore);
                                                alpha = alpha.max(eval);
                                                board.unmake_move();
        
                                                if alpha > beta {
                                                    break;
                                                }
                                            },
                                            Err(TimeoutError) => {
                                                return Err(TimeoutError);
                                            }
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
                                        board.make_move(m);
                                        match self.alpha_beta(board, alpha, beta, depth - 1) {
                                            Ok(newscore) => {
                                                beta = beta.min(newscore);
                                                board.unmake_move();

                                                if alpha > beta {
                                                    break;
                                                }
                                            },
                                            Err(TimeoutError) => {
                                                return Err(TimeoutError);
                                            }
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
                    self.table.set(board.bitboard(), eval);
                }
                return Ok(eval);
            }
        }
    }

    fn move_list(&mut self, board: &mut Board, prev_ml: &Vec<Move>, depth: u8) -> Result<Vec<Move>, TimeoutError> {
        let mut alpha = MIN;
        let mut beta = MAX;
        let mut out: Vec<Move> = Vec::new();

        match board.player() {
            Player::P1 => {
                for m in prev_ml {
                    match self.timer.check() {
                        Ok(_) => {
                            match m.score().state {
                                GameState::OPEN => {
                                    board.make_move(m.col());
                                    match self.alpha_beta(board, alpha, beta, depth - 1) {
                                        Ok(newscore) => {
                                            out.push(Move::new(m.col(), m.player(), newscore, depth));
                                            alpha = alpha.max(newscore);
                                            board.unmake_move();
                                        },
                                        Err(TimeoutError) => {
                                            return Err(TimeoutError);
                                        }
                                    }
                                },
                                _ => {
                                    out.push(m.clone());
                                }
                            }
                            return Ok(out);
                        },
                        Err(TimeoutError) => {
                            return Err(TimeoutError);
                        }
                    }
                }
            },
            Player::P2 => {
                for m in prev_ml {
                    match self.timer.check() {
                        Ok(_) => {
                            match m.score().state {
                                GameState::OPEN => {
                                    board.make_move(m.col());
                                    match self.alpha_beta(board, alpha, beta, depth - 1) {
                                        Ok(newscore) => {
                                            out.push(Move::new(m.col(), m.player(), newscore, depth));
                                            beta = beta.min(newscore);
                                            board.unmake_move();
                                        },
                                        Err(TimeoutError) => {
                                            return Err(TimeoutError);
                                        }
                                    }
                                },
                                _ => {
                                    out.push(m.clone());
                                }
                            }
                            return Ok(out);
                        },
                        Err(TimeoutError) => {
                            return Err(TimeoutError);
                        }
                    }
                }
            }
        }
        return Err(TimeoutError)
    }

    fn init_move_array(board: &Board) -> Vec<Move> {
        let mut out: Vec<Move> = Vec::new();
        let cols = board.legal_moves();
        for c in cols {
            out.push(Move::new(c, board.player(), EQUAL , 0));
        }
        out
    }

    pub fn iterative_depening(&mut self, board: &Board) -> Move {
        let mut tb: Board = board.clone();
        let mut movelist = Self::init_move_array(&tb);
        let mut bestmove: Move = movelist[0];
        let cells: u8 = 7*6;
        for i in 1..cells {
            self.table.clean();
            match self.move_list(&mut tb, &movelist, i) {
                Ok(mut ml) => {
                    ml.sort();
                    if board.player() == Player::P1 {
                        ml.reverse();
                        movelist = ml;
                        bestmove = movelist[0];
                    }
                },
                Err(TimeoutError) => {
                    return bestmove;
                }
            }
        }
        return bestmove;
    }
}