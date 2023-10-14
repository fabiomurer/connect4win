use crate::bit_board::*;
use crate::score::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
use std::hash::*;

#[derive(Clone, Default)]
pub struct Entry {
    score: Score,
    key: BitBoard,
}

pub struct Table {
    size: usize,
    table: Box<[Entry]>,
}

impl Table {
    pub fn new(size: usize) -> Table {
        Table {
            size,
            table: vec![Default::default(); 1].into_boxed_slice(),
        }
    }

    pub fn alloc(&mut self) {
        self.table = vec![Default::default(); self.size].into_boxed_slice();
    }

    pub fn clean(&mut self) {
        self.alloc();
    }

    pub fn get_ready(&mut self) {
        self.alloc();
    }

    fn get_index(&self, key: &BitBoard) -> usize {
        let mut s = DefaultHasher::new();
        key.hash(&mut s);
        let n: usize = s.finish() as usize;
        n % self.size
    }

    pub fn get(&self, key: &DoubleBitBoard) -> Option<Score> {
        let index1 = self.get_index(&key.board());
        if self.table[index1].key == key.board() {
            Some(self.table[index1].score)
        } else {
            // check if is stored as a mirrored position
            let index2 = self.get_index(&key.board_mirrored());
            if self.table[index2].key == key.board_mirrored() {
                Some(self.table[index2].score)
            } else {
                None
            }
        }
    }

    pub fn set(&mut self, key: DoubleBitBoard, score: Score) {
        let index = self.get_index(&key.board());
        let entry: Entry = Entry {
            score,
            key: key.board(),
        };
        self.table[index] = entry;
    }

    pub fn set_size(&mut self, size: usize) {
        self.size = size;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn getset() {
        let mut table = Table::new(100);
        table.get_ready();
        let mut bitboard = DoubleBitBoard::new();
        bitboard.make_move(0, &crate::board::Player::P1);
        table.set(bitboard, EQUAL);
        let sc = table.get(&bitboard).unwrap();
        assert_eq!(sc, EQUAL);
    }
}
