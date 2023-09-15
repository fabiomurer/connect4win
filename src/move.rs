use std::cmp::Ordering;
use std::fmt::Display;

use crate::board::*;
use crate::score::*;

#[derive(Debug, Clone, Copy)]
pub struct Move {
    col: u8,
    player: Player,
    score: Score,
    depth: u8,
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Move {
    pub fn new(col: u8, player: Player, score: Score, depth: u8) -> Self {
        Self {
            col,
            player,
            score,
            depth,
        }
    }

    pub fn col(&self) -> u8 {
        self.col
    }

    pub fn score(&self) -> Score {
        self.score
    }

    pub fn player(&self) -> Player {
        self.player
    }
}

impl Eq for Move {}

impl PartialEq for Move {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Move {
    fn cmp(&self, other: &Self) -> Ordering {
        let compare_score = self.score.cmp(&other.score);

        if compare_score == Ordering::Equal && self.score.state == GameState::Open {
            let col: i32 = 7;
            let half_row = col / 2;
            let distance_a = ((self.col as i32) - half_row).abs();
            let distance_b = ((other.col as i32) - half_row).abs();
            distance_b.cmp(&distance_a)
        } else if compare_score != Ordering::Equal {
            compare_score
        } else {
            let compare_depth = self.depth.cmp(&other.depth);
            match self.score.state {
                GameState::Open => Ordering::Equal,
                GameState::Draw => compare_depth,
                GameState::WinP1 => compare_depth.reverse(),
                GameState::WinP2 => compare_depth,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cmp() {
        let m1 = Move::new(3, Player::P1, EQUAL, 2);
        let m2 = Move::new(0, Player::P1, EQUAL, 2);
        assert_eq!(true, m1 > m2);
        let m1 = Move::new(3, Player::P1, MIN, 10);
        let m2 = Move::new(0, Player::P1, MIN, 2);
        assert_eq!(true, m1 > m2);
        let m1 = Move::new(3, Player::P1, MAX, 10);
        let m2 = Move::new(0, Player::P1, MAX, 2);
        assert_eq!(true, m1 < m2);

        let m1 = Move::new(
            4,
            Player::P2,
            Score {
                score: -4,
                state: GameState::WinP1,
            },
            2,
        );
        let m2 = Move::new(
            6,
            Player::P2,
            Score {
                score: -3,
                state: GameState::WinP1,
            },
            18,
        );
        assert_eq!(true, m1.cmp(&m2) == Ordering::Greater);
    }
}
