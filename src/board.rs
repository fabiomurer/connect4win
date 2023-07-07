use std::fmt;

const COL:u64 = 7;
const ROW:u64 = 6;
const CONNECT:u64 = 4;

const INIT_BITBOARD:u64 = 0b0_110_110_110_110_110_110_110_000000000000000000000000000000000000000000;

pub enum GameState {
    OPEN,
    WINP1,
    WINP2,
}

pub enum Player {
    P1,
    P2,
}

#[derive(Default)]
struct BitBoard {
    board: u64,
}

const SIZE_SPACE:u64 = 3;
impl BitBoard {
    pub fn get_space(&self, col: u64) -> u64 {
        let offset = COL*ROW + SIZE_SPACE*col; 
        let mut bits: u64 = 0b111 << offset;
        bits = self.board & bits;
        bits = bits >> offset;
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
        let offset = COL*ROW + SIZE_SPACE*col;
        let inc: u64 = 0b1 << offset;
        self.board += inc;
    }
    fn dec_space(&mut self, col: u64) {
        let offset = COL*ROW + SIZE_SPACE*col;
        let inc: u64 = 0b1 << offset;
        self.board -= inc;
    }
    fn set_stone(&mut self, col: u64, row: u64, player: Player) {
        let offset = row*COL + col;
        match player {
            Player::P2 => self.board ^= 0b1 << offset,
            Player::P1 => ()
        }
    }

    pub fn make_move(&mut self, col: u64, player: Player) {
        self.set_stone(col, ROW - self.get_space(col), player);
        self.dec_space(col);
    }
    pub fn unmake_move(&mut self, col: u64, player: Player) {
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
                    let offset: u64 = row*COL + (col as u64);
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

    pub fn init()  -> BitBoard {
        BitBoard { board: INIT_BITBOARD }
    }
}

#[cfg(test)]
mod tests {
    use crate::board::INIT_BITBOARD;

    use super::BitBoard;

    #[test]
    fn init() {
        let mut b:BitBoard = BitBoard::init();
        assert_eq!(b.get_space(6), 6);
        assert_eq!(b.get_space(0), 6);

        b.make_move(3, crate::board::Player::P1);
        assert_eq!(b.get_space(3), 5);
        b.unmake_move(3, crate::board::Player::P1);
        assert_eq!(b.get_space(3), 6);

        b.print();
        b.make_move(0, crate::board::Player::P1);
        assert_eq!(b.board, 0b0_110_110_110_110_110_110_101_000000000000000000000000000000000000000000);
        b.print();
        b.make_move(0, crate::board::Player::P2);
        assert_eq!(b.board, 0b0_110_110_110_110_110_110_100_000000000000000000000000000000000010000000);
        b.print();
        b.unmake_move(0, crate::board::Player::P2);
        assert_eq!(b.board, 0b0_110_110_110_110_110_110_101_000000000000000000000000000000000000000000);
        b.unmake_move(0, crate::board::Player::P1);
        assert_eq!(b.board, INIT_BITBOARD);
    }
}