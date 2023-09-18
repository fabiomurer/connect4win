use yew::prelude::*;

use crate::board::*;
use crate::move_engine::*;
use crate::r#move::*;
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

#[derive(Properties, PartialEq)]
struct BoardProps {
    board: Board,
}

#[function_component]
fn GameBoard(props: &BoardProps) -> Html {
    let arr = board_to_arr(&props.board);

    html! {
        <>
        <table style="width:500px">
            {
                arr.into_iter().map(|r| {
                    html! {
                        <tr style="height:50px">
                            {
                                r.into_iter().map(|c| {
                                    match c {
                                        CellType::Empty => html!{<td style="background-color: blue">{""}</td>},
                                        CellType::P1 => html!{<td style="background-color: red">{""}</td>},
                                        CellType::P2 => html!{<td style="background-color: yellow">{""}</td>}
                                    }
                                }).collect::<Html>()
                            }
                        </tr>
                    }

                }).collect::<Html>()
            }
        </table>
        <p>{format!("game state: {:?}", props.board.gamestate())}</p>
        </>
    }
}

enum Msg {
    NewGame,
    Bestmove,
    Move(u8),
}

struct App {
    engine: Engine,
    board: Board,
    m: Move,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            engine: Engine::new(3, 100_000),
            board: Board::new(),
            m: Move::new(0, Player::P1, EQUAL, 0),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::NewGame => {
                self.board = Board::new();
                self.m = Move::new(0, Player::P1, EQUAL, 0);
                true
            }
            Msg::Bestmove => {
                self.m = self.engine.iterative_depening(&self.board);
                self.board.make_move(self.m.col());
                true
            }
            Msg::Move(col) => {
                self.board.make_move(col);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let spaces = self.board.bitboard().get_space_array();
        html! {
            <div>
                <button onclick={ctx.link().callback(|_| Msg::NewGame)}>{ "new game" }</button>
                <button onclick={ctx.link().callback(|_| Msg::Bestmove)}>{ "play best move" }</button>
                <p>{ self.m }</p>
                {
                    html! {
                        if self.board.gamestate() == GameState::Open {
                            <table style="width:500px">
                                <tr>
                                    {
                                        (0..(7 as u8)).map(|i| {
                                            if spaces[i as usize] > 0 {
                                                html! { <td><button onclick={ctx.link().callback(move |_| Msg::Move(i))}> { "move" }</button></td>}
                                            } else {
                                                html! { <td><button onclick={ctx.link().callback(move |_| Msg::Move(i))} disabled={true}> { "move" }</button></td>}
                                            }
                                        }).collect::<Html>()
                                    }
                                </tr>
                            </table>
                        }
                    }
                }
                <GameBoard board={self.board.clone()}/>
            </div>
        }
    }
}

pub fn app() {
    yew::Renderer::<App>::new().render();
}
