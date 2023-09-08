use std::cmp::Ordering;

use crate::board::*;
use crate::score::*;

#[derive(Debug, Clone, Copy)]
pub struct Move {
    col: u8,
    player: Player,
    score: Score,
    depth: u8,
}

impl Move {
    pub fn new(col: u8, player: Player, score: Score, depth: u8) -> Self { Self { col, player, score, depth } }

    pub fn col(&self) -> u8 {
        self.col
    }

    pub fn score(&self) -> Score {
        self.score
    }

    pub fn player(&self) -> Player {
        self.player
    }

    pub fn depth(&self) -> u8 {
        self.depth
    }
}

impl Eq for Move {}

impl PartialEq for Move {
    fn eq(&self, other: &Self) -> bool {
        return self == other;
    }
}

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

impl Ord for Move {
    fn cmp(&self, other: &Self) -> Ordering {
        let compare_score = self.score.cmp(&other.score);

        if compare_score == Ordering::Equal && self.score.state == GameState::OPEN {
            let col: i32 = 7;
            let half_row = col / 2;
            let distance_a = ((self.col as i32) - half_row).abs();
            let distance_b = ((other.col as i32) - half_row).abs();
            return distance_b.cmp(&distance_a);
        } else {
            if compare_score != Ordering::Equal {
                return compare_score;
            } else {
                let compare_depth = self.depth.cmp(&other.depth);
                match self.score.state {
                    GameState::OPEN  => Ordering::Equal,
                    GameState::DRAW  => compare_depth,
                    GameState::WINP1 => compare_depth.reverse(),
                    GameState::WINP2 => compare_depth,
                }
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
    }
}