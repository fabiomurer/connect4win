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
        let saved_score: Option<Score> = if depth >= 2 {
            self.table.get(&board.bitboard())
        } else {
            None
        };

        match  saved_score {
            Some(score) => Ok(score),
            None => {
                let moves: Vec<u8>;
                let mut eval: Score;

                if depth == 0 || board.gamestate() != GameState::Open {
                    return Ok(board.evaluate());
                } else {
                    if depth >= 1 {
                        moves = Self::move_sort(board);
                    } else {
                        moves = board.legal_moves();
                    }

                    match board.player() {
                        Player::P1 => {
                            eval = MIN;
                            for m in moves {
                                self.timer.check()?;

                                board.make_move(m);
                                let newscore = self.alpha_beta(board, alpha, beta, depth - 1)?;
                                board.unmake_move();

                                eval = eval.max(newscore);
                                alpha = alpha.max(eval);
                                if alpha > beta {
                                    break;
                                }
                            }
                        },
                        Player::P2 => {
                            eval = MAX;
                            for m in moves {
                                self.timer.check()?;

                                board.make_move(m);
                                let newscore = self.alpha_beta(board, alpha, beta, depth - 1)?;
                                board.unmake_move();

                                eval = eval.min(newscore);
                                beta = beta.min(eval);
                                if alpha > beta {
                                    break;
                                }
                            }
                        }
                    }
                }
                if depth >= 2 {
                    self.table.set(board.bitboard(), eval);
                }
                Ok(eval)
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
                    self.timer.check()?;
                    match m.score().state {
                        GameState::Open => {
                            board.make_move(m.col());
                            let newscore = self.alpha_beta(board, alpha, beta, depth - 1)?;
                            board.unmake_move();

                            out.push(Move::new(m.col(), m.player(), newscore, depth));
                            alpha = alpha.max(newscore);
                        },
                        _ => {
                            out.push(*m);
                        }
                    }
                }
            },
            Player::P2 => {
                for m in prev_ml {
                    self.timer.check()?;
                    match m.score().state {
                        GameState::Open => {
                            board.make_move(m.col());
                            let newscore = self.alpha_beta(board, alpha, beta, depth - 1)?;
                            board.unmake_move();

                            out.push(Move::new(m.col(), m.player(), newscore, depth));
                            beta = beta.min(newscore);
                        },
                        _ => {
                            out.push(*m);
                        }
                    }
                }
            }
        }
        Ok(out)
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
        self.timer.start();
        let mut tb: Board = board.clone();
        let mut movelist = Self::init_move_array(&tb);
        let mut bestmove: Move = movelist[0];
        let cells: u8 = board.free_cells();
        for i in 1..cells {
            self.table.clean();
            match self.move_list(&mut tb, &movelist, i) {
                Ok(mut ml) => {
                    ml.sort();
                    if board.player() == Player::P1 {
                        ml.reverse();
                    }
                    movelist = ml;
                    bestmove = movelist[0];

                    println!("Move list");
                    for m in &movelist {
                        println!("{:?}", m);
                    }
                    println!("");
                    
                },
                Err(TimeoutError) => {
                    return bestmove;
                }
            }
        }
        bestmove
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ml() {
        let mut b = Board::new();
        let ml = Engine::move_sort(&mut b);
        assert_eq!(ml, [3, 4, 2, 5, 1, 6, 0])
    }
}