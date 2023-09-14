use yew::prelude::*;

use crate::board::*;
use crate::move_engine::*;
use crate::r#move::*;
use crate::score::*;

enum Msg {
    Start,
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

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Start => {
                self. m = self.engine.iterative_depening(&self.board);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <button onclick={ctx.link().callback(|_| Msg::Start)}>{ "start" }</button>
                <p>{ self.m.col() }</p>
            </div>
        }
    }

}

pub fn app() {
    yew::Renderer::<App>::new().render();
}