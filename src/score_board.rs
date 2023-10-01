use crate::board::*;
use crate::get_array;
use crate::push;
use crate::showtype;
use crate::vec_macro;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScoreSet {
    score: i32,
    p1: u32,
    p2: u32,
}

impl ScoreSet {
    fn fix_score(&mut self) {
        if self.p1 == 0 || self.p2 == 0 {
            if self.p1 > self.p2 {
                self.score = self.p1 as i32;
            } else {
                self.score = -(self.p2 as i32);
            }
        } else {
            self.score = 0;
        }
    }
    fn add(&mut self, player: &Player) {
        match player {
            Player::P1 => self.p1 += 1,
            Player::P2 => self.p2 += 1,
        }
        self.fix_score();
    }
    fn sub(&mut self, player: &Player) {
        match player {
            Player::P1 => self.p1 -= 1,
            Player::P2 => self.p2 -= 1,
        }
        self.fix_score();
    }

    pub const fn init() -> ScoreSet {
        ScoreSet {
            score: 0,
            p1: 0,
            p2: 0,
        }
    }
}

const NSC: usize = 207; // chissà se è giosto

#[derive(Clone, PartialEq)]
pub struct ScoreBoard {
    total_score: i32,
    scoreboard: [[showtype!(u32, 16); COL as usize]; ROW as usize],
    scoresets: [ScoreSet; NSC],
}

impl ScoreBoard {
    pub fn total_score(&self) -> i32 {
        self.total_score
    }

    pub fn make_move(&mut self, row: usize, col: usize, player: &Player) -> bool {
        let mut delta_score: i32 = 0;
        let mut win: bool = false;

        for i in get_array!(self.scoreboard[row][col]) {
            let sc = &mut self.scoresets[*i as usize];
            let ps = sc.score;
            sc.add(player);

            if sc.score.abs() == CONNECT as i32 {
                win = true;
            }

            delta_score += sc.score - ps;
        }
        self.total_score += delta_score;
        win
    }

    pub fn unmake_move(&mut self, row: usize, col: usize, player: &Player) {
        let mut delta_score: i32 = 0;

        for i in get_array!(self.scoreboard[row][col]) {
            let sc = &mut self.scoresets[*i as usize];
            let ps = sc.score;
            sc.sub(player);

            delta_score += sc.score - ps;
        }
        self.total_score += delta_score;
    }

    pub const fn new() -> ScoreBoard {
        let sca: [ScoreSet; NSC] = [ScoreSet::init(); NSC];

        let mut sbt: [[showtype!(u32, 16); COL as usize]; ROW as usize] =
            [[vec_macro!(0 as u32, 16); COL as usize]; ROW as usize];
        let mut n: u32 = 0;
        let mut i: u64 = 0;
        while i < ROW {
            let mut j: u64 = 0;
            while j < COL {
                if ROW - i >= CONNECT {
                    // orizontal --
                    let mut k = i;
                    while k < (i + CONNECT) {
                        push!(sbt[k as usize][j as usize], n);
                        k += 1;
                    }
                    n += 1;
                }

                if COL - j >= CONNECT {
                    // vertical |
                    let mut k = j;
                    while k < (j + CONNECT) {
                        push!(sbt[i as usize][k as usize], n);
                        k += 1;
                    }
                    n += 1;
                }

                if (ROW - i >= CONNECT) && (COL - j >= CONNECT) {
                    // diagonal \
                    let mut k = i;
                    let mut kk = j;
                    while k < (i + CONNECT) {
                        push!(sbt[k as usize][kk as usize], n);
                        k += 1;
                        kk += 1;
                    }
                    n += 1;
                }

                if (ROW - i >= CONNECT) && (j + 1 >= CONNECT) {
                    // diagonal /
                    let mut k = i;
                    let mut kk = j;
                    while k < (i + CONNECT) {
                        push!(sbt[k as usize][kk as usize], n);
                        k += 1;
                        kk = kk.saturating_sub(1);
                    }
                    n += 1;
                }
                j += 1;
            }
            i += 1;
        }

        let sb = sbt;
        ScoreBoard {
            total_score: 0,
            scoreboard: sb,
            scoresets: sca,
        }
    }
}

pub const SCORE_BOARD: ScoreBoard = ScoreBoard::new();

#[cfg(test)]
mod tests {
    use super::ScoreBoard;

    #[test]
    fn lel() {
        let mut ss: ScoreBoard = ScoreBoard::new();
        ss.make_move(0, 0, &crate::board::Player::P1);
        assert_eq!(ss.total_score, 3);
        ss.make_move(0, 6, &crate::board::Player::P2);
        assert_eq!(ss.total_score, 0);
        let mut ss: ScoreBoard = ScoreBoard::new();
        ss.make_move(0, 0, &crate::board::Player::P2);
        assert_eq!(ss.total_score, -3);
        ss.make_move(0, 6, &crate::board::Player::P1);
        assert_eq!(ss.total_score, 0)
    }

    #[test]
    fn tm() {
        let mut ss: ScoreBoard = ScoreBoard::new();
        ss.make_move(0, 0, &crate::board::Player::P1);
        ss.unmake_move(0, 0, &crate::board::Player::P1);
        assert_eq!(ss.total_score, 0)
    }

    #[test]
    fn win() {
        let mut ss: ScoreBoard = ScoreBoard::new();
        ss.make_move(0, 0, &crate::board::Player::P1);
        ss.make_move(1, 0, &crate::board::Player::P1);
        ss.make_move(2, 0, &crate::board::Player::P1);
        assert!(ss.make_move(3, 0, &crate::board::Player::P1));

        let mut ss: ScoreBoard = ScoreBoard::new();
        ss.make_move(0, 0, &crate::board::Player::P2);
        ss.make_move(1, 0, &crate::board::Player::P2);
        ss.make_move(2, 0, &crate::board::Player::P2);
        assert!(ss.make_move(3, 0, &crate::board::Player::P2));
    }
}
