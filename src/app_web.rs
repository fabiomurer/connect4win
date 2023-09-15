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
    let  arr = board_to_arr(&props.board);

    html! {
        <table style="width:500px">
            {
                arr.into_iter().map(|r| {
                    html! {
                        <tr style="height:50px">
                            {
                                r.into_iter().map(|c| {
                                    match c {
                                        CellType::Empty => html!{<th style="background-color: blue">{""}</th>},
                                        CellType::P1 => html!{<th style="background-color: red">{""}</th>},
                                        CellType::P2 => html!{<th style="background-color: yellow">{""}</th>}
                                    }
                                }).collect::<Html>()
                            }
                        </tr>
                    }
                    
                }).collect::<Html>()
            }
        </table>
    }
}

enum Msg {
    Start,
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
        Self { engine: Engine::new(3, 100_000), board: Board::new(), m: Move::new(0, Player::P1, EQUAL , 0) }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Start => {
                self.m = self.engine.iterative_depening(&self.board);
                true
            }
            Msg::Move(col) => {
                self.board.make_move(col);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <button onclick={ctx.link().callback(|_| Msg::Start)}>{ "start" }</button>
                <button onclick={ctx.link().callback(|_| Msg::Move(0))}>{ "move 1" }</button>
                <button onclick={ctx.link().callback(|_| Msg::Move(1))}>{ "move 2" }</button>
                <button onclick={ctx.link().callback(|_| Msg::Move(2))}>{ "move 3" }</button>
                <button onclick={ctx.link().callback(|_| Msg::Move(3))}>{ "move 4" }</button>
                <button onclick={ctx.link().callback(|_| Msg::Move(4))}>{ "move 5" }</button>
                <button onclick={ctx.link().callback(|_| Msg::Move(5))}>{ "move 6" }</button>
                <button onclick={ctx.link().callback(|_| Msg::Move(6))}>{ "move 7" }</button>
                <p>{ self.m.col() }</p>
                <GameBoard board={self.board.clone()}/>
            </div>
        }
    }

}

pub fn app() {
    yew::Renderer::<App>::new().render();
}