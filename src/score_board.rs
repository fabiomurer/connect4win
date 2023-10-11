use crate::board::*;
use std::collections::LinkedList;

const WEIGHTS: [[i32; COL as usize]; ROW as usize] = [
    [0, 1, 2, 3, 2, 1, 0],
    [0, 1, 2, 3, 2, 1, 0],
    [0, 1, 2, 3, 2, 1, 0],
    [0, 1, 2, 3, 2, 1, 0],
    [0, 1, 2, 3, 2, 1, 0],
    [0, 1, 2, 3, 2, 1, 0],
];

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

    pub fn init() -> ScoreSet {
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
    scoreboard: [[LinkedList<u32>; COL as usize]; ROW as usize],
    scoresets: [ScoreSet; NSC],
}

impl ScoreBoard {
    pub fn total_score(&self) -> i32 {
        self.total_score
    }

    pub fn make_move(&mut self, row: usize, col: usize, player: &Player) -> bool {
        let mut delta_score: i32 = 0;
        let mut win: bool = false;

        for i in self.scoreboard[row][col].iter() {
            let sc = &mut self.scoresets[*i as usize];
            let ps = sc.score;
            sc.add(player);

            if sc.score.abs() == CONNECT as i32 {
                win = true;
            }

            delta_score += sc.score - ps;
        }
        self.total_score += delta_score * 10;
        match player {
            Player::P1 => self.total_score += WEIGHTS[row][col],
            Player::P2 => self.total_score -= WEIGHTS[row][col],
        }
        win
    }

    pub fn unmake_move(&mut self, row: usize, col: usize, player: &Player) {
        let mut delta_score: i32 = 0;

        for i in self.scoreboard[row][col].iter() {
            let sc = &mut self.scoresets[*i as usize];
            let ps = sc.score;
            sc.sub(player);

            delta_score += sc.score - ps;
        }
        self.total_score += delta_score * 10;
        match player {
            Player::P1 => self.total_score -= WEIGHTS[row][col],
            Player::P2 => self.total_score += WEIGHTS[row][col],
        }
    }

    pub fn new() -> ScoreBoard {
        let sca: [ScoreSet; NSC] = [ScoreSet::init(); NSC];

        let mut sbt: [[LinkedList<u32>; COL as usize]; ROW as usize] = [
            [
                LinkedList::new(),
                LinkedList::new(),
                LinkedList::new(),
                LinkedList::new(),
                LinkedList::new(),
                LinkedList::new(),
                LinkedList::new(),
            ],
            [
                LinkedList::new(),
                LinkedList::new(),
                LinkedList::new(),
                LinkedList::new(),
                LinkedList::new(),
                LinkedList::new(),
                LinkedList::new(),
            ],
            [
                LinkedList::new(),
                LinkedList::new(),
                LinkedList::new(),
                LinkedList::new(),
                LinkedList::new(),
                LinkedList::new(),
                LinkedList::new(),
            ],
            [
                LinkedList::new(),
                LinkedList::new(),
                LinkedList::new(),
                LinkedList::new(),
                LinkedList::new(),
                LinkedList::new(),
                LinkedList::new(),
            ],
            [
                LinkedList::new(),
                LinkedList::new(),
                LinkedList::new(),
                LinkedList::new(),
                LinkedList::new(),
                LinkedList::new(),
                LinkedList::new(),
            ],
            [
                LinkedList::new(),
                LinkedList::new(),
                LinkedList::new(),
                LinkedList::new(),
                LinkedList::new(),
                LinkedList::new(),
                LinkedList::new(),
            ],
        ];
        let mut n: u32 = 0;
        for i in 0..ROW {
            for j in 0..COL {
                if ROW - i >= CONNECT {
                    // orizontal --
                    for k in i..(i + CONNECT) {
                        sbt[k as usize][j as usize].push_back(n);
                    }
                    n += 1;
                }

                if COL - j >= CONNECT {
                    // vertical |
                    for k in j..(j + CONNECT) {
                        sbt[i as usize][k as usize].push_back(n);
                    }
                    n += 1;
                }

                if (ROW - i >= CONNECT) && (COL - j >= CONNECT) {
                    // diagonal \
                    let mut kk = j;
                    for k in i..(i + CONNECT) {
                        sbt[k as usize][kk as usize].push_back(n);
                        kk += 1;
                    }
                    n += 1;
                }

                if (ROW - i >= CONNECT) && (j + 1 >= CONNECT) {
                    // diagonal /
                    let mut kk = j;
                    for k in i..(i + CONNECT) {
                        sbt[k as usize][kk as usize].push_back(n);
                        kk = kk.saturating_sub(1);
                    }
                    n += 1;
                }
            }
        }

        let sb = sbt;
        ScoreBoard {
            total_score: 0,
            scoreboard: sb,
            scoresets: sca,
        }
    }
}

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
