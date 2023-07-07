use std::collections::{linked_list, LinkedList};

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

struct MoveStack {
    moves: LinkedList<u8>,
}

impl MoveStack {
    pub fn push_move(&mut self, col: u8) {
        self.moves.push_front(col);
    }
    pub fn pop_move(&mut self, col: u8) -> u8 {
        self.moves.pop_front().unwrap()
    }
    pub fn init()  -> MoveStack {
        MoveStack { moves: LinkedList::new() }
    }
}

#[derive (Clone, Copy)]
struct ScoreSet {
    score: i32,
    p1: i32,
    p2: i32
}

impl ScoreSet {
    fn fix_score(&mut self) {
        if self.p1 == 0 || self.p2 == 0 {
            if self.p1 > self.p2 {
                self.score = self.p1;
            } else {
                self.score = -self.p2
            }
        } else {
            self.score = 0;
        }
    }
    fn add(&mut self, player: Player) {
        match player {
            Player::P1 => self.p1 += 1,
            Player::P2 => self.p2 += 1
        }
        self.fix_score();
    }
    fn sub(&mut self, player: Player) {
        match player {
            Player::P1 => self.p1 -= 1,
            Player::P2 => self.p2 -= 1
        }
        self.fix_score();
    }
    
    pub fn init() -> ScoreSet {
        ScoreSet { score: 0, p1: 0, p2: 0 }
    }
}

#[derive (Clone)]
struct ScoreBoard {
    scoreboard: [[LinkedList<ScoreSet>; ROW as usize]; COL as usize]
}

impl ScoreBoard {
    
    pub fn init() -> ScoreBoard {
        let mut sb: [[LinkedList<ScoreSet>; ROW as usize]; COL as usize] = 
        [
            [LinkedList::new(), LinkedList::new(), LinkedList::new(), LinkedList::new(), LinkedList::new(), LinkedList::new()],
            [LinkedList::new(), LinkedList::new(), LinkedList::new(), LinkedList::new(), LinkedList::new(), LinkedList::new()],
            [LinkedList::new(), LinkedList::new(), LinkedList::new(), LinkedList::new(), LinkedList::new(), LinkedList::new()],
            [LinkedList::new(), LinkedList::new(), LinkedList::new(), LinkedList::new(), LinkedList::new(), LinkedList::new()],
            [LinkedList::new(), LinkedList::new(), LinkedList::new(), LinkedList::new(), LinkedList::new(), LinkedList::new()],
            [LinkedList::new(), LinkedList::new(), LinkedList::new(), LinkedList::new(), LinkedList::new(), LinkedList::new()],
            [LinkedList::new(), LinkedList::new(), LinkedList::new(), LinkedList::new(), LinkedList::new(), LinkedList::new()]
        ];
        for i in 0..ROW {
            for j in 0..COL {
                if ROW - i >= CONNECT {
                    let sc: ScoreSet = ScoreSet::init();
                    for k in i..CONNECT {
                        sb[k as usize][j as usize].push_back(sc);
                    }
                }
                if COL - j >= CONNECT {
                    let sc: ScoreSet = ScoreSet::init();
                    for k in j..CONNECT {
                        sb[i as usize][k as usize].push_back(sc);
                    }
                }
                if (ROW - i >= CONNECT) && (COL - j >= CONNECT) {
                    let sc: ScoreSet = ScoreSet::init();
                    let mut kk = j;
                    for k in i..CONNECT {
                        sb[k as usize][kk as usize].push_back(sc);
                        kk += 1;
                    }
                }
                if (i + 1 >= CONNECT) && (COL - j >= CONNECT) {
                    let sc: ScoreSet = ScoreSet::init();
                    let mut kk = j;
                    
                    let mut k = i;
                    while i - k < CONNECT {
                        sb[k as usize][kk as usize].push_back(sc);
                        k -= 1;
                        kk += 1;
                    }
                }
            }
        }

        ScoreBoard { scoreboard: sb }
    }
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

struct Board {
    movestack: MoveStack,
    bitboard: BitBoard,
    gamestate: GameState,
    player: Player
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