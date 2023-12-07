# connect4win
a connect-four engine for cli and [web](https://zwirgioilredelcodice.github.io/connet4win-web/)

## build and run

* build the database witn `RUST_MIN_STACK=10485760000 cargo test make_game_database -- --nocapture`

### CLI
* dependencies `rustup`, `cargo`
* build `cargo build --release`
* run `cargo run ` or `./target/release/connect4win`

### WEB
* dependencies `rustup`, `cargo`, `rustup target add wasm32-unknown-unknown`, `cargo install dioxus-cli --locked`
* run `dx serve --release`