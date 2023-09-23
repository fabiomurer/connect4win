mod app_console;
mod app_web;
mod bit_board;
mod board;
mod r#move;
mod move_engine;
mod score;
mod score_board;
mod timer;
mod transposition_table;

#[cfg(target_family = "wasm")]
extern crate console_error_panic_hook;
#[cfg(target_family = "wasm")]
use std::panic;

#[cfg(target_family = "wasm")]
fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    app_web::app();
}

#[cfg(not(target_family = "wasm"))]
fn main() {
    app_console::app();
}
