use std::cmp::Ordering;

use crate::board::*;
use crate::score::*;

pub struct Move {
    col: u8,
    player: Player,
    score: Score,
    depth: u8,
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
            return distance_a.cmp(&distance_b);
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
        