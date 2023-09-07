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
    let board = board::Board::new();
    let mut e = move_engine::Engine::new(10, 100_000);
    let m = e.iterative_depening(&board);
    println!("{:?}", m);
}
