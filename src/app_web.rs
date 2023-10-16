#![cfg(target_family = "wasm")]
#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::board::*;
use crate::move_engine::*;
use crate::r#move::Move;
use crate::score::*;

#[derive(Debug, Clone, Copy)]
enum CellType {
    P1,
    P2,
    Empty,
}

fn board_to_arr(board: &Board) -> [[CellType; COL as usize]; ROW as usize] {
    let mut arr = [[CellType::Empty; COL as usize]; ROW as usize];
    let bb = board.bitboard();
    let spacearr = bb.get_space_array();

    for row in 0..ROW {
        for col in 0..COL {
            let rrow = ROW - row - 1;
            if ROW - spacearr[col as usize] <= row {
                arr[rrow as usize][col as usize] = CellType::Empty;
            } else {
                let offset: u64 = row * COL + (col as u64);
                let bit = (bb.board().board() & (0b1 << offset)) >> (offset);
                if bit == 0 {
                    arr[rrow as usize][col as usize] = CellType::P1;
                } else {
                    arr[rrow as usize][col as usize] = CellType::P2;
                }
            }
        }
    }
    arr
}

fn Board(cx: Scope) -> Element {
    let board = use_shared_state::<Board>(cx).unwrap();
    let arr = board_to_arr(&board.read());
    let spaces = board.read().bitboard().get_space_array();
    let state = board.read().gamestate();
    cx.render(rsx! {
        div {
            "style": "max-width: 50em; margin: auto;",
            p {"game state: {state:?}"},
            div {
                table {
                    "style": "table-layout: fixed; width: 100%; height: 100%; text-align: center;",
                    tr {
                        (0..(7 as u8)).map(|i| {
                            rsx! {
                                td { 
                                    button {
                                        disabled: (spaces[i as usize] <= 0) || (board.read().gamestate() != GameState::Open),
                                        onclick: move |_| {
                                            board.write().make_move(i)
                                        },
                                        "move",
                                    }
                                }
                            } 
                        })
                    }
                }
            },
            div {
                "style": "aspect-ratio : 7 / 6; background-color: black;",
                table {
                    "style": "table-layout: fixed; width: 100%; height: 100%; border-spacing: 0.5em;",
                    arr.into_iter().map(|r| rsx! {
                        tr {
                            r.into_iter().map(|c| rsx! {
                                match c {
                                    CellType::Empty => rsx! { td { "style": "border-radius: 50%; background-color: blue"}},
                                    CellType::P1 => rsx! { td { "style": "border-radius: 50%; background-color: yellow"}},
                                    CellType::P2 => rsx! { td { "style": "border-radius: 50%; background-color: red"}},
                                }
                            })
                        }
                    })
                }
            }
            div {
                button {
                    onclick: move |_| {
                        *board.write() = Board::new();
                    },
                    "reset",
                }
            },
            div {
                button {
                    disabled: board.read().is_empty(), 
                    onclick: move |_| {
                        board.write().unmake_move();
                    },
                    "undo",
                }
            },
        }
    })
}

#[derive(Props)]
struct LayoutProps<'a> {
    children: Element<'a>,
}

fn Layout<'a>(cx: Scope<'a, LayoutProps<'a>>) -> Element {
    cx.render(rsx! {
        body {
            class: "HolyGrail",
            header {
                "style": "margin: auto;",
                h1 { "Connect4Win" },
                p { "A connect-four game engine" }
            },
            div {
                class: "HolyGrail-body",
                r#main {
                    class: "HolyGrail-content",
                    &cx.props.children
                },
                nav {
                    class: "HolyGrail-nav"
                },
                aside {
                    class: "HolyGrail-ads"
                }
            },
            footer {}
        }
    })
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum PlayerType {
    Human,
    Cpu,
}

impl PlayerType {
    fn rev(self) -> PlayerType {
        match self {
            PlayerType::Cpu => PlayerType::Human,
            PlayerType::Human => PlayerType::Cpu,
        }
    }
}

const DEFAULT_SECS: u64 = 3;
const DEFAULT_TABLE_SIZE: usize = 100_000;

fn App(cx: Scope) -> Element {
    use_shared_state_provider(cx, || Board::new());
    let board = use_shared_state::<Board>(cx).unwrap();

    let e1 = use_ref(cx, || Engine::new(DEFAULT_SECS, DEFAULT_TABLE_SIZE));
    let e2 = use_ref(cx, || Engine::new(DEFAULT_SECS, DEFAULT_TABLE_SIZE));
    let p1 = use_state(cx, || PlayerType::Human);
    let p2 = use_state(cx, || PlayerType::Human);

    let p1t = use_state(cx, || DEFAULT_SECS);
    let p1m = use_state(cx, || DEFAULT_TABLE_SIZE);
    let p2t = use_state(cx, || DEFAULT_SECS);
    let p2m = use_state(cx, || DEFAULT_TABLE_SIZE);

    let m1 = use_state(cx, || Move::new(0, Player::P1, EQUAL, 0));
    let m2 = use_state(cx, || Move::new(0, Player::P1, EQUAL, 0));

    cx.render(rsx! {
        Layout {
            div {
                input {
                    id: "p1",
                    r#type: "checkbox",
                    oninput: |_| p1.set(p1.rev()),
                }
                label {
                    "for": "p1",
                    "Cpu player 1"
                }

                if *p1.get() == PlayerType::Cpu {
                    rsx! {
                        div {
                            p {"{m1}"},
                            input {
                                r#type: "number",
                                id: "p1t",
                                "min": 1,
                                value: "{p1t}",
                                oninput: move |evt| {
                                    if !evt.value.is_empty() {
                                        let n: u64 = match evt.value.parse() {
                                            Err(_) => *p1t.get(),
                                            Ok(num) => num
                                        };
                                        p1t.set(n);
                                        e1.with_mut(|e1| e1.set_time(n))
                                    }
                                }
                            }
                            label {
                                "for": "p1t",
                                "Time (s)"
                            }
                            br {}
                            input {
                                r#type: "number",
                                id: "p1m",
                                "min": 1_000,
                                "max": 1_000_000,
                                "step": 50_000,
                                value: "{p1m}",
                                oninput: move |evt| {
                                    if !evt.value.is_empty() {
                                        let n: usize = match evt.value.parse() {
                                            Err(_) => DEFAULT_TABLE_SIZE,
                                            Ok(num) => num
                                        };
                                        p1m.set(n);
                                        e1.with_mut(|e1| e1.set_table(n))
                                    }
                                }
                            }
                            label {
                                "for": "p1m",
                                "Table size (entrys)"
                            }
                            br {}
                            if board.read().player() == Player::P1 {
                                rsx! {
                                    button {
                                        "style": "color: red;",
                                        onclick: move |_| {
                                            let m = e1.with_mut(|e| e.iterative_depening(&board.read()));
                                            board.write().make_move(m.col());
                                            m1.set(m);
                                        },
                                        "compute move"
                                    }
                                }
                            }
                        }
                    }
                }
            }

            div {
                input {
                    id: "p2",
                    r#type: "checkbox",
                    oninput: |_| p2.set(p2.rev()),
                }
                label {
                    "for": "p2",
                    "Cpu player 2"
                }

                if *p2.get() == PlayerType::Cpu {
                    rsx! {
                        p {"{m2}"},
                        div {
                            input {
                                r#type: "number",
                                id: "p2t",
                                "min": 1,
                                value: "{p2t}",
                                oninput: move |evt| {
                                    if !evt.value.is_empty() {
                                        let n: u64 = match evt.value.parse() {
                                            Err(_) => *p1t.get(),
                                            Ok(num) => num
                                        };
                                        p2t.set(n);
                                        e2.with_mut(|e2| e2.set_time(n))
                                    }
                                }
                            }
                            label {
                                "for": "p2t",
                                "Time (s)"
                            }
                            br {}
                            input {
                                r#type: "number",
                                id: "p2m",
                                "min": 1_000,
                                "max": 1_000_000,
                                "step": 50_000,
                                value: "{p2m}",
                                oninput: move |evt| {
                                    if !evt.value.is_empty() {
                                        let n: usize = match evt.value.parse() {
                                            Err(_) => DEFAULT_TABLE_SIZE,
                                            Ok(num) => num
                                        };
                                        p2m.set(n);
                                        e2.with_mut(|e2| e2.set_table(n))
                                    }
                                }
                            }
                            label {
                                "for": "p2m",
                                "Table size (entrys)"
                            }
                            br {}
                            if board.read().player() == Player::P2 {
                                rsx! {
                                    button {
                                        "style": "color: red;",
                                        onclick: move |_| {
                                            let m = e2.with_mut(|e| e.iterative_depening(&board.read()));
                                            board.write().make_move(m.col());
                                            m2.set(m);
                                        },
                                        "compute move"
                                    }
                                }
                            }
                        }
                    }
                }
            }

            Board {}
        }
    })
}

pub fn app() {
    dioxus_web::launch(App);
}
