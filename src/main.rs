mod board;
mod score_board;
mod bit_board;
mod score;
mod r#move;
mod transposition_table;
mod timer;
mod move_engine;
mod app_console;
mod app_web;

fn main() {
    if cfg!(target_family = "wasm") {
        app_web::app(); 
    } else {
        app_console::app();
    }
}
