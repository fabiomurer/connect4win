use crate::board::*;
use crate::r#move::*;
use crate::score::*;
use crate::timer::*;
use crate::transposition_table::*;

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

    pub fn set_time(&mut self, seconds: u64) {
        self.timer.set_duration(seconds);
    }

    pub fn set_table(&mut self, table_size: usize) {
        self.table.set_size(table_size);
    }

    fn move_sort(board: &mut Board) -> Vec<u8> {
        let v = board.legal_moves();
        let mut s: Vec<(Score, u8)> = Vec::with_capacity(v.len());
        let mut out: Vec<u8> = Vec::with_capacity(v.len());

        for m in v {
            board.make_move(m);
            s.push((-board.evaluate(), m));
            board.unmake_move();
        }
        s.sort_by_key(|k| k.0);

        for m in s {
            out.push(m.1);
        }
        out
    }

    pub fn alpha_beta(
        &mut self,
        board: &mut Board,
        mut alpha: Score,
        beta: Score,
        depth: u8,
    ) -> Result<Score, TimeoutError> {
        let saved_score: Option<Score> = if depth >= 1 {
            self.table.get(&board.bitboard())
        } else {
            None
        };

        match saved_score {
            Some(score) => Ok(score),
            None => {
                let moves: Vec<u8>;
                let mut bestscore: Score = MIN;

                if depth == 0 || board.gamestate() != GameState::Open {
                    return Ok(board.evaluate());
                } else {
                    if depth >= 1 {
                        //moves = Self::move_sort(board); fa piu casino
                        moves = board.legal_moves();
                    } else {
                        moves = board.legal_moves();
                    }
                    for m in moves {
                        self.timer.check()?;

                        board.make_move(m);
                        let score = -self.alpha_beta(board, -beta, -alpha, depth - 1)?;
                        board.unmake_move();

                        if score >= beta {
                            bestscore = score;
                            break;
                        }
                        if score > bestscore {
                            bestscore = score;
                            if score > alpha {
                                alpha = score
                            }
                        }
                    }
                }
                if depth >= 1 {
                    self.table.set(board.bitboard(), bestscore);
                }
                Ok(bestscore)
            }
        }
    }

    fn move_list(
        &mut self,
        board: &mut Board,
        prev_ml: &Vec<Move>,
        depth: u8,
    ) -> Result<Vec<Move>, TimeoutError> {
        let alpha = MIN;
        let beta = MAX;
        let mut out: Vec<Move> = Vec::with_capacity(COL as usize);

        for m in prev_ml {
            self.timer.check()?;

            board.make_move(m.col());
            let newscore = self.alpha_beta(board, alpha, beta, depth - 1)?;
            board.unmake_move();

            out.push(Move::new(m.col(), m.player(), newscore, depth));
            //alpha = alpha.max(newscore);
        }
        Ok(out)
    }

    fn init_move_array(board: &Board) -> Vec<Move> {
        let mut out: Vec<Move> = Vec::with_capacity(COL as usize);
        let cols = board.legal_moves();
        for c in cols {
            out.push(Move::new(c, board.player(), EQUAL, 0));
        }
        out
    }

    fn get_ready(&mut self) {
        self.table.get_ready();
        self.timer.start();
    }

    pub fn iterative_depening(&mut self, board: &Board) -> Move {
        self.get_ready();

        let mut tb: Board = board.clone();
        let mut movelist = Self::init_move_array(&tb);
        let mut bestmove: Move = movelist[0];
        let cells: u8 = board.free_cells();

        if board.is_empty() {
            return Move::new(3, board.player(), EQUAL, 0);
        }

        for i in 1..cells {
            self.table.clean();
            match self.move_list(&mut tb, &movelist, i) {
                Ok(mut ml) => {
                    ml.sort();
                    movelist = ml;
                    bestmove = *movelist.last().unwrap();
                }
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

    #[test]
    fn timeengine() {
        use std::time::Instant;

        let mut board = Board::new();
        board.make_move(3);
        let mut e = Engine::new(3, 100_000);
        e.get_ready();

        let start = Instant::now();
        _ = e.alpha_beta(&mut board, MIN, MAX, 12);
        let duration = start.elapsed();
        println!("Time elapsed in alpha_beta is: {:?}", duration);
    }
}
