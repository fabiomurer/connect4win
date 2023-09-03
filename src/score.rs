use crate::board::*;
use std::cmp::Ordering;

pub struct Score {
    pub score: i32,
    pub state: GameState,
}

impl Eq for Score {}

impl PartialEq for Score {
    fn eq(&self, other: &Self) -> bool {
        if self.state == other.state {
            return true;
        } else {
            return self.score == other.score;
        }
    }
}

impl PartialOrd for Score {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return Some(self.cmp(other));
    }
}

impl Ord for Score {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.state == other.state {
            return self.state.cmp(&other.state);
        } else {
            return self.score.cmp(&other.score);
        }
    }
}