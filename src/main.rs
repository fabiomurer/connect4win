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

//#[cfg(target_family = "wasm")]
extern crate console_error_panic_hook;
//#[cfg(target_family = "wasm")]
use std::panic;

fn main() {
    if cfg!(target_family = "wasm") {
        panic::set_hook(Box::new(console_error_panic_hook::hook));
        app_web::app(); 
    } else {
        app_console::app();
    }
}
