use crate::board::*;
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy)]
pub struct Score {
    pub score: i32,
    pub state: GameState,
}

impl Score {
    pub fn new(score: i32, state: GameState) -> Self { Self { score, state } }
}

pub const MAX: Score = Score { score: 0, state: GameState::WinP1 };
pub const MIN: Score = Score { score: 0, state: GameState::WinP2 };
pub const EQUAL: Score = Score { score: 0, state: GameState::Open };

impl Default for Score {
    fn default() -> Self {
        Score { score: 0, state: GameState::Open }
    }
}

impl Eq for Score {}

impl PartialEq for Score {
    fn eq(&self, other: &Self) -> bool {
        if self.state == other.state {
            true
        } else {
            self.score == other.score
        }
    }
}

impl PartialOrd for Score {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Score {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.state == other.state {
            self.score.cmp(&other.score)
        } else {
            self.state.cmp(&other.state)
        }
    }

    fn max(self, other: Self) -> Self
        where
            Self: Sized, {
        if self > other {
            self
        } else {
            other
        }
    }
    
    fn min(self, other: Self) -> Self
        where
            Self: Sized, {
        if self < other {
            self
        } else {
            other
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cmpp() {
        let s1 = MAX;
        let s2 = EQUAL;
        assert_eq!(MAX, s1.max(s2));

        let s1 = Score { score: 12, state: GameState::Open };
        let s2 = Score { score: 0, state: GameState::Open };
        assert_eq!(s1, s2.max(s1));

        let s1 = Score { score: 12, state: GameState::Draw };
        let s2 = Score { score: 0, state: GameState::Open };
        assert_eq!(s2, s2.max(s1));
    }
}