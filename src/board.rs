use crate::bit_board::*;
use crate::score::*;
use crate::score_board::*;

pub const COL: u64 = 7;
pub const ROW: u64 = 6;
pub const CONNECT: u64 = 4;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub enum GameState {
    #[default]
    Open,
    Draw,
    WinP1,
    WinP2,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum Player {
    P1,
    P2,
}

#[derive(Clone, PartialEq)]
struct MoveStack {
    moves: Vec<u8>,
}

impl MoveStack {
    pub fn push_move(&mut self, col: u8) {
        self.moves.push(col);
    }
    pub fn pop_move(&mut self) -> u8 {
        self.moves.pop().unwrap()
    }
    pub fn new() -> MoveStack {
        MoveStack {
            moves: Vec::with_capacity((COL * ROW) as usize),
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct Board {
    movestack: MoveStack,
    bitboard: DoubleBitBoard,
    scoreboard: ScoreBoard,
    gamestate: GameState,
    player: Player,
}

impl Board {
    pub fn make_move(&mut self, col: u8) {
        if col as u64 > COL {
            panic!("this collumn does not exist")
        } else if self.bitboard.get_space(col as u64) < 1 {
            panic!("collumn full")
        } else if self.gamestate != GameState::Open {
            panic!("game closed")
        }

        let row = ROW - self.bitboard.get_space(col as u64);
        self.movestack.push_move(col);
        self.bitboard.make_move(col as u64, &self.player);
        let win = self
            .scoreboard
            .make_move(row as usize, col as usize, &self.player);
        if win {
            match self.player {
                Player::P1 => self.gamestate = GameState::WinP1,
                Player::P2 => self.gamestate = GameState::WinP2,
            }
        } else if self.bitboard.is_full() {
            self.gamestate = GameState::Draw
        } else {
            self.gamestate = GameState::Open
        }

        match self.player {
            Player::P1 => self.player = Player::P2,
            Player::P2 => self.player = Player::P1,
        }
    }
    pub fn unmake_move(&mut self) {
        self.gamestate = GameState::Open;
        match self.player {
            Player::P1 => self.player = Player::P2,
            Player::P2 => self.player = Player::P1,
        }

        let col = self.movestack.pop_move();
        self.bitboard.unmake_move(col as u64, &self.player);

        let row = ROW - self.bitboard.get_space(col as u64);
        self.scoreboard
            .unmake_move(row as usize, col as usize, &self.player);
    }

    pub fn legal_moves(&self) -> Vec<u8> {
        let mut v: Vec<u8> = Vec::with_capacity(COL as usize);
        let spaces = self.bitboard.get_space_array();
        for (col, i) in spaces.iter().enumerate() {
            if *i > 0 {
                v.push(u8::try_from(col).unwrap());
            }
        }
        v
    }

    pub fn nmoves(&self) -> u8 {
        self.movestack.moves.len() as u8
    }

    pub fn evaluate(&self) -> Score {
        getscore(self)
    }

    pub fn free_cells(&self) -> u8 {
        7 * 6 - (self.movestack.moves.len() as u8)
    }

    pub fn is_empty(&self) -> bool {
        self.bitboard.is_empty()
    }

    pub fn new() -> Board {
        Board {
            movestack: MoveStack::new(),
            bitboard: DoubleBitBoard::new(),
            scoreboard: ScoreBoard::new(),
            gamestate: GameState::Open,
            player: Player::P1,
        }
    }

    pub fn set_from_pos_string(&mut self, pos: &str) {
        *self = Board::new();
        for c in pos.chars() {
            let col: u8 = c.to_digit(10).unwrap() as u8;
            self.make_move(col)
        }
    }

    pub fn from_position(pos: &str) -> Board {
        let mut b = Board::new();
        b.set_from_pos_string(pos);
        b
    }

    pub fn player(&self) -> Player {
        self.player
    }

    pub fn bitboard(&self) -> DoubleBitBoard {
        self.bitboard
    }

    pub fn gamestate(&self) -> GameState {
        self.gamestate
    }

    pub fn scoreboard(&self) -> &ScoreBoard {
        &self.scoreboard
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn munm() {
        let mut b = Board::new();
        b.make_move(3);
        b.unmake_move();

        assert_eq!(b.bitboard(), DoubleBitBoard::new());
        assert_eq!(b.movestack.moves.len(), 0);
        assert_eq!(b.scoreboard.total_score(), 0);
        assert_eq!(b.player, Player::P1);
        assert_eq!(b.gamestate, GameState::Open);
    }
}
