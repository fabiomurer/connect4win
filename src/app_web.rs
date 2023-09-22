#![cfg(target_family = "wasm")]
#![allow(non_snake_case)]

use dioxus::html::br;
use dioxus::html::div;
use dioxus::html::input;
use dioxus::html::label;
use dioxus::html::p;
use dioxus::prelude::*;

use crate::board::*;
use crate::move_engine::*;

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
                let bit = (bb.board() & (0b1 << offset)) >> (offset);
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
        p {"game state: {state:?}"},
        div {
            table {
                "style": "width: 500px",
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
            table {
                "style": "width: 500px",
                arr.into_iter().map(|r| rsx! {
                    tr {
                        "style": "height:50px",
                        r.into_iter().map(|c| rsx! {
                            match c {
                                CellType::Empty => rsx! { td { "style": "background-color: blue"}},
                                CellType::P1 => rsx! { td { "style": "background-color: yellow"}},
                                CellType::P2 => rsx! { td { "style": "background-color: red"}},
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
    })
}

fn Intro(cx: Scope) -> Element {
    cx.render(rsx! {
        h1 { "Connect4Win" },
        p { "A connect-four game engine" }
    })
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum PlaterType {
    Human,
    Cpu,
}

impl PlaterType {
    fn rev(self) -> PlaterType {
        match self {
            PlaterType::Cpu => PlaterType::Human,
            PlaterType::Human => PlaterType::Cpu,
        }
    }
}

const DEFAULT_SECS: u64 = 3;
const DEFAULT_TABLE_SIZE: usize = 100_000;

fn App(cx: Scope) -> Element {
    use_shared_state_provider(cx, || Board::new());
    let board = use_shared_state::<Board>(cx).unwrap();

    let mut e1 = Engine::new(DEFAULT_SECS, DEFAULT_TABLE_SIZE);
    let mut e2 = Engine::new(DEFAULT_SECS, DEFAULT_TABLE_SIZE);

    let mut p1 = use_state(cx, || PlaterType::Human);
    let mut p2 = use_state(cx, || PlaterType::Human);

    cx.render(rsx! {
        div {
            Intro {},

            
            /*button {
                onclick: move |_| {
                    let m = e.iterative_depening(&board.read());
                    board.write().make_move(m.col())
                },
                "cpu move",
            },*/
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

                if *p1.get() == PlaterType::Cpu {
                    rsx! {
                        div {
                            input {
                                r#type: "number",
                                id: "p1t",
                                "min": 1,
                                value: 3,
                                oninput: move |evt| {
                                    e1.set_time(evt.value.parse().unwrap());
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
                                value: 100_000,
                                oninput: move |evt| {
                                    e1.set_table(evt.value.parse().unwrap());
                                }
                            }
                            label {
                                "for": "p1m",
                                "Table size (entrys)"
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

                if *p2.get() == PlaterType::Cpu {
                    rsx! {
                        div {
                            input {
                                r#type: "number",
                                id: "p2t",
                                "min": 1,
                                value: 3,
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
                                value: 100_000,
                            }
                            label {
                                "for": "p2m",
                                "Table size (entrys)"
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