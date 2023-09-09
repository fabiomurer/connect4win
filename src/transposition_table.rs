use std::collections::hash_map::DefaultHasher;
use std::hash::*;
use std::hash::Hasher;
use crate::bit_board::*;
use crate::score::*;

#[derive(Clone, Default)]
pub struct Entry {
    score: Score,
    key: BitBoard
}

pub struct Table {
    size: usize,
    table: Box<[Entry]>
}

const DEFAULT_SIZE: usize = 100_000;


impl Table {
    pub fn new(size: usize) -> Table {
        
        Table { 
            size, 
            table: vec![Default::default(); size].into_boxed_slice(),
        }
    }

    pub fn clean(&mut self) {
        self.table = vec![Default::default(); self.size].into_boxed_slice();
    }

    fn get_index(&self, key: &BitBoard) -> usize {
        let mut s = DefaultHasher::new();
        key.hash(&mut s);
        let n: usize = s.finish() as usize;
        n % self.size
    }

    pub fn get(&self, key: &BitBoard) -> Option<Score> {
        let index = self.get_index(key);
        if self.table[index].key == *key {
            Some(self.table[index].score)
        } else {
            None
        }
    }

    pub fn set(&mut self, key: BitBoard, score: Score) {
        let index = self.get_index(&key);
        let entry: Entry = Entry { score, key };
        self.table[index] = entry;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn getset() {
        let mut table = Table::new(100);
        let mut bitboard = BitBoard::new();
        bitboard.make_move(0, &crate::board::Player::P1);
        table.set(bitboard, EQUAL);
        let sc = table.get(&bitboard).unwrap();
        assert_eq!(sc, EQUAL);
    }
}