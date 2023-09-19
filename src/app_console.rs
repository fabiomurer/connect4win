use crate::board;
use crate::move_engine;
use std::io;

pub fn app() {
    let mut board = board::Board::new();
    let mut e = move_engine::Engine::new(3_000_000, 100_000);

    println!("p1 for first player, p2 for second");
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let human = match buf.trim() {
        "p1" => board::Player::P1,
        "p2" => board::Player::P2,
        _ => panic!("player option not valid"),
    };

    while board.gamestate() == board::GameState::Open {
        if board.player() == human {
            let mut buf = String::new();
            io::stdin().read_line(&mut buf).unwrap();
            let col: u8 = buf.trim().parse().unwrap();

            board.make_move(col);
        } else {
            let m = e.iterative_depening(&board);
            board.make_move(m.col());
            println!("{:?}", m);
        }

        board.bitboard().print();
    }
    println!("{:?}", board.gamestate());
}
