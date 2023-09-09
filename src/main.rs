use crate::board::GameState;
use std::io;

mod board;
mod score_board;
mod bit_board;
mod score;
mod r#move;
mod transposition_table;
mod timer;
mod move_engine;
fn main() {
    println!("Hello, world!");
    let mut board = board::Board::new();
    let mut e = move_engine::Engine::new(3, 100_000);

    while board.gamestate() == GameState::Open {
        match board.player() {
            board::Player::P1 => {
                let mut buf = String::new();
                io::stdin().read_line(&mut buf).unwrap();
                let col: u8 = buf.trim().parse().unwrap();
        
                board.make_move(col);
            }, 
            board::Player::P2 => {
                let m = e.iterative_depening(&board);
                board.make_move(m.col());

                println!("{:?}", m);
            }
        }
        board.bitboard().print();
    }
    println!("{:?}", board.gamestate());
}
