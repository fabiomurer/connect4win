use crate::board::*;
use std::hash::Hash;

#[cfg(not(target_family = "wasm"))]
use serde::{Deserialize, Serialize};

const INIT_BITBOARD: u64 =
    0b0_110_110_110_110_110_110_110_000000000000000000000000000000000000000000;

#[cfg(not(target_family = "wasm"))]
#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub struct BitBoard {
    board: u64,
}

#[cfg(target_family = "wasm")]
#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug, PartialOrd, Ord)]
pub struct BitBoard {
    board: u64,
}

impl Default for BitBoard {
    fn default() -> Self {
        BitBoard { board: 0 }
    }
}

const SIZE_SPACE: u64 = 3;
impl BitBoard {
    pub fn is_full(&self) -> bool {
        (self.board & 0b0_111_111_111_111_111_111_111_000000000000000000000000000000000000000000)
            == 0b0_111_111_111_111_111_111_111_000000000000000000000000000000000000000000
    }
    pub fn is_empty(&self) -> bool {
        self.board == INIT_BITBOARD
    }
    pub fn get_space(&self, col: u64) -> u64 {
        let offset = COL * ROW + SIZE_SPACE * col;
        let mut bits: u64 = 0b111 << offset;
        bits &= self.board;
        bits >>= offset;
        bits
    }
    pub fn get_space_array(&self) -> [u64; 7] {
        let mut arr: [u64; 7] = [0; 7];
        for i in 0..7 {
            arr[i] = self.get_space(i as u64);
        }
        arr
    }
    fn inc_space(&mut self, col: u64) {
        let offset = COL * ROW + SIZE_SPACE * col;
        let inc: u64 = 0b1 << offset;
        self.board += inc;
    }
    fn dec_space(&mut self, col: u64) {
        let offset = COL * ROW + SIZE_SPACE * col;
        let inc: u64 = 0b1 << offset;
        self.board -= inc;
    }
    fn set_stone(&mut self, col: u64, row: u64, player: &Player) {
        let offset = row * COL + col;
        match player {
            Player::P2 => self.board ^= 0b1 << offset,
            Player::P1 => (),
        }
    }

    pub fn make_move(&mut self, col: u64, player: &Player) {
        self.set_stone(col, ROW - self.get_space(col), player);
        self.dec_space(col);
    }
    pub fn unmake_move(&mut self, col: u64, player: &Player) {
        self.inc_space(col);
        self.set_stone(col, ROW - self.get_space(col), player);
    }

    pub fn print(&self) {
        let spacearr = self.get_space_array();

        println!("board {:#b}", self.board);
        for row in (0..6).rev() {
            for col in 0..7 {
                if ROW - spacearr[col] <= row {
                    print!(". ");
                } else {
                    let offset: u64 = row * COL + (col as u64);
                    let bit = (self.board & (0b1 << offset)) >> (offset);
                    if bit == 0 {
                        print!("O ");
                    } else {
                        print!("X ");
                    }
                }
            }
            println!(" ");
        }
    }

    pub fn new() -> BitBoard {
        BitBoard {
            board: INIT_BITBOARD,
        }
    }

    pub fn board(&self) -> u64 {
        self.board
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub struct DoubleBitBoard {
    pub(crate) normal: BitBoard,
    pub(crate) mirrored: BitBoard,
}

impl DoubleBitBoard {
    pub fn new() -> Self {
        DoubleBitBoard {
            normal: BitBoard::new(),
            mirrored: BitBoard::new(),
        }
    }
    pub fn is_full(&self) -> bool {
        self.normal.is_full()
    }
    pub fn is_empty(&self) -> bool {
        self.normal.is_empty()
    }
    pub fn get_space(&self, col: u64) -> u64 {
        self.normal.get_space(col)
    }
    pub fn get_space_array(&self) -> [u64; 7] {
        self.normal.get_space_array()
    }
    pub fn make_move(&mut self, col: u64, player: &Player) {
        self.normal.make_move(col, player);
        self.mirrored.make_move((COL - 1) - col, player);
    }
    pub fn unmake_move(&mut self, col: u64, player: &Player) {
        self.normal.unmake_move(col, player);
        self.mirrored.unmake_move((COL - 1) - col, player);
    }
    pub fn print(&self) {
        self.normal.print();
    }

    pub fn board(&self) -> BitBoard {
        self.normal
    }
    pub fn board_mirrored(&self) -> BitBoard {
        self.mirrored
    }
}

#[cfg(test)]
mod tests {
    use super::BitBoard;

    #[test]
    fn init() {
        let mut b: BitBoard = BitBoard::new();
        assert_eq!(b.get_space(6), 6);
        assert_eq!(b.get_space(0), 6);

        b.make_move(3, &crate::board::Player::P1);
        assert_eq!(b.get_space(3), 5);
        b.unmake_move(3, &crate::board::Player::P1);
        assert_eq!(b.get_space(3), 6);

        b.print();
        b.make_move(0, &crate::board::Player::P1);
        assert_eq!(
            b.board,
            0b0_110_110_110_110_110_110_101_000000000000000000000000000000000000000000
        );
        b.print();
        b.make_move(0, &crate::board::Player::P2);
        assert_eq!(
            b.board,
            0b0_110_110_110_110_110_110_100_000000000000000000000000000000000010000000
        );
        b.print();
        b.unmake_move(0, &crate::board::Player::P2);
        assert_eq!(
            b.board,
            0b0_110_110_110_110_110_110_101_000000000000000000000000000000000000000000
        );
        b.unmake_move(0, &crate::board::Player::P1);
        assert_eq!(b.board, super::INIT_BITBOARD);
    }
}
