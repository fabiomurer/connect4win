use crate::bit_board::BitBoard;
use crate::score::*;
use bincode;
use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;

const ENTRYS: usize = 4200899;

#[derive(Clone, Default, Serialize, Deserialize, Copy)]
struct Entry {
    score: Score,
    key: BitBoard,
}

#[derive(Serialize, Deserialize)]
pub struct GameDatabase {
    #[serde(with = "BigArray")]
    data: [Entry; ENTRYS],
}

impl GameDatabase {
    pub fn new() -> GameDatabase {
        let data: [Entry; ENTRYS] = [Entry::default(); ENTRYS];
        GameDatabase { data: data }
    }
    pub fn insert(&mut self, entry: &Entry, index: usize) {
        self.data[index] = *entry;
    }
}

#[cfg(test)]
mod test {
    use crate::score::EQUAL;

    use super::*;
    use std::fs::File;
    use std::io::{self, BufRead, BufWriter};
    use std::path::Path;
    use std::str::FromStr;

    const DBPATH: &str = "./database/db-12ply-distance.txt";
    const DBOUT: &str = "./database/db-12ply-distance";
    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
    {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }

    fn line_to_entry(line: &str) -> Entry {
        let token: Vec<&str> = line.split(" ").collect();

        let mut b: BitBoard = BitBoard::new();
        let mut col = 0;
        for c in token[0].chars() {
            match c {
                '.' => col += 1,
                '1' => b.make_move(col, &crate::board::Player::P1),
                '2' => b.make_move(col, &crate::board::Player::P2),
                _ => panic!("what is this char"),
            }
        }
        let s: i32 = FromStr::from_str(token[1]).unwrap();
        let dist = (100 - s.abs()) + 12;

        let score: Score;
        if s == 0 {
            score = EQUAL;
        } else if s > 0 {
            score = W1 - dist;
        } else {
            score = W2 + dist;
        }

        return Entry {
            score: score,
            key: b,
        };
    }

    #[test]
    fn make_game_database() {
        let mut nlines = 0;
        let mut gd = GameDatabase::new();
        if let Ok(lines) = read_lines(DBPATH) {
            // Consumes the iterator, returns an (Optional) String
            for (i, line) in lines.enumerate() {
                if let Ok(ip) = line {
                    let e = line_to_entry(ip.as_str());
                    gd.insert(&e, i)
                }
                nlines = i;
            }
        }
        let sas = bincode::serialize(&gd).unwrap();
        println!("size is (byte){}, line readed {}", sas.len(), nlines);

        let mut fout = BufWriter::new(File::create(DBOUT).unwrap());
        bincode::serialize_into(&mut fout, &sas).unwrap();

        assert_eq!(ENTRYS - 1, nlines)
    }
}
