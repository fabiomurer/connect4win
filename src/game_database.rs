use crate::bit_board::*;
use crate::score::*;
use bincode;
use serde::{Deserialize, Serialize};

use std::fs::*;
use std::io::BufReader;

pub const PLY: u8 = 12;
pub const GOOD_QUERY: u8 = PLY + 1;

#[allow(dead_code)]
const ENTRYS: usize = 4200899;

#[allow(dead_code)]
const DBIN: &str = "./database/db-12ply-distance.txt";
const DBOUT: &str = "./database/db-12ply-distance";

#[derive(Clone, Default, Serialize, Deserialize, Copy)]
struct Entry {
    score: Score,
    key: BitBoard,
}

#[derive(Serialize, Deserialize)]
pub struct GameDatabase {
    data: Vec<Entry>,
}

impl GameDatabase {
    pub fn new() -> GameDatabase {
        let file_db = File::open(DBOUT).unwrap();
        let reader = BufReader::new(file_db);
        let data = bincode::deserialize_from(reader).unwrap();
        GameDatabase { data }
    }

    pub fn get(&self, key: &DoubleBitBoard) -> Option<Score> {
        match self.data.binary_search_by_key(&key.board(), |e| e.key) {
            Ok(index) => Some(self.data[index].score),
            Err(_) => {
                match self
                    .data
                    .binary_search_by_key(&key.board_mirrored(), |e| e.key)
                {
                    Ok(index) => Some(self.data[index].score),
                    Err(_) => None,
                }
            }
        }
    }

    #[allow(dead_code)]
    fn set_data(&mut self, mut data: Vec<Entry>) {
        data.sort_by_key(|e| e.key);
        self.data = data;
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use std::fs::File;
    use std::io::{self, BufRead, BufWriter};
    use std::path::Path;
    use std::str::FromStr;

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
            score = DRAW + dist;
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

    // RUST_MIN_STACK=10485760000 cargo test make_game_database -- --nocapture
    /// convert db-12ply-distance.txt to binary file ready to use
    #[test]
    fn make_game_database() {
        let mut nlines = 0;
        let mut data: Vec<Entry> = Vec::with_capacity(ENTRYS);
        if let Ok(lines) = read_lines(DBIN) {
            // Consumes the iterator, returns an (Optional) String
            for (i, line) in lines.enumerate() {
                if let Ok(ip) = line {
                    let e = line_to_entry(ip.as_str());
                    data.push(e);
                }
                nlines = i;
            }
        }
        let mut gd = GameDatabase { data: Vec::new() };
        gd.set_data(data);

        let mut fout = BufWriter::new(File::create(DBOUT).unwrap());
        bincode::serialize_into(&mut fout, &gd).unwrap();

        assert_eq!(ENTRYS - 1, nlines)
    }

    #[test]
    fn getto() {
        let db = GameDatabase::new();
        let e = line_to_entry("1.....12112.212212. 79");
        let bb = DoubleBitBoard {
            normal: e.key,
            mirrored: e.key,
        };
        assert_eq!(db.get(&bb).unwrap(), e.score)
    }
}
