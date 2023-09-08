use crate::bit_board;
use crate::bit_board::BitBoard;
use crate::score_board;
use std::collections::LinkedList;
use std::cmp::Ordering;
use crate::score::*;

pub const COL: u64 = 7;
pub const ROW: u64 = 6;
pub const CONNECT: u64 = 4;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GameState {
    OPEN,
    DRAW,
    WINP1,
    WINP2,
}

impl PartialOrd for GameState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}
impl Ord for GameState {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            GameState::OPEN => {
                match other {
                    GameState::OPEN  => Ordering::Equal,
                    GameState::DRAW  => Ordering::Greater,
                    GameState::WINP1 => Ordering::Less,
                    GameState::WINP2 => Ordering::Greater,
                }
            }
            GameState::DRAW => {
                match other {
                    GameState::OPEN  => Ordering::Less,
                    GameState::DRAW  => Ordering::Equal,
                    GameState::WINP1 => Ordering::Less,
                    GameState::WINP2 => Ordering::Greater,
                }
            }
            GameState::WINP1 => {
                match other {
                    GameState::OPEN  => Ordering::Greater,
                    GameState::DRAW  => Ordering::Greater,
                    GameState::WINP1 => Ordering::Equal,
                    GameState::WINP2 => Ordering::Greater,
                }
            }
            GameState::WINP2 => {
                match other {
                    GameState::OPEN  => Ordering::Less,
                    GameState::DRAW  => Ordering::Less,
                    GameState::WINP1 => Ordering::Less,
                    GameState::WINP2 => Ordering::Equal,
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum Player {
    P1,
    P2,
}

#[derive(Clone)]
struct MoveStack {
    moves: LinkedList<u8>,
}

impl MoveStack {
    pub fn push_move(&mut self, col: u8) {
        self.moves.push_front(col);
    }
    pub fn pop_move(&mut self) -> u8 {
        self.moves.pop_front().unwrap()
    }
    pub fn new() -> MoveStack {
        MoveStack {
            moves: LinkedList::new(),
        }
    }
}

#[derive(Clone)]
pub struct Board {
    movestack: MoveStack,
    bitboard: bit_board::BitBoard,
    scoreboard: score_board::ScoreBoard,
    gamestate: GameState,
    player: Player,
}

impl Board {
    pub fn make_move(&mut self, col: u8) {
        if col as u64 > COL {
            panic!("this collumn does not exist")
        } else if self.bitboard.get_space(col as u64) < 1 {
            panic!("collumn full")
        } else if self.gamestate != GameState::OPEN {
            panic!("game closed")
        }

        let row = ROW - self.bitboard.get_space(col as u64);
        self.movestack.push_move(col);
        self.bitboard.make_move(col as u64, &self.player);
        let win = self.scoreboard.make_move(row as usize, col as usize, &self.player);
        if win {
            match self.player {
                Player::P1 => self.gamestate = GameState::WINP1,
                Player::P2 => self.gamestate = GameState::WINP2
            }
        } else if self.bitboard.is_full() {
            self.gamestate = GameState::DRAW
        } else {
            self.gamestate = GameState::OPEN
        }

        match self.player {
            Player::P1 => self.player = Player::P2,
            Player::P2 => self.player = Player::P1,
        }

    }
    pub fn unmake_move(&mut self) {
        self.gamestate = GameState::OPEN;
        match self.player {
            Player::P1 => self.player = Player::P2,
            Player::P2 => self.player = Player::P1,
        }
        
        let col = self.movestack.pop_move();
        let row = ROW - self.bitboard.get_space(col as u64);

        self.bitboard.unmake_move(col as u64, &self.player);
        self.scoreboard.unmake_move(row as usize, col as usize, &self.player);
    }

    pub fn legal_moves(&self) -> Vec<u8> {
        let mut v: Vec<u8> = Vec::new();
        let spaces = self.bitboard.get_space_array();
        for (col, i) in spaces.iter().enumerate() {
            if *i > 0 {
                v.push(u8::try_from(col).unwrap());
            }
        }
        v
    }

    pub fn evaluate(&self) -> Score {
        Score { 
            score: self.scoreboard.total_score(), 
            state: self.gamestate.clone(),
        }
    }

    pub fn new() -> Board {
        Board { 
            movestack: MoveStack::new(), 
            bitboard: bit_board::BitBoard::new(),
            scoreboard: score_board::ScoreBoard::new(), 
            gamestate: GameState::OPEN, 
            player: Player::P1 
        }
    }

    pub fn player(&self) -> Player {
        self.player
    }

    pub fn bitboard(&self) -> BitBoard {
        self.bitboard
    }

    pub fn gamestate(&self) -> GameState {
        self.gamestate
    }
}