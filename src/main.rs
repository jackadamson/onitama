#![feature(array_map)]

mod models;
mod cards;
mod game;

use yew::prelude::*;
use models::*;
use yew::virtual_dom::VNode;

enum Msg {
    SelectCell { x: usize, y: usize },
    SelectCard { card: Card },
}

struct Model {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    state: GameState,
    selected_square: Option<Point>,
    selected_card: Option<Card>,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let state = GameState::new();
        log::info!("Game state: {:?}", &state);
        Self {
            link,
            state,
            selected_square: None,
            selected_card: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let board = match &self.state {
            GameState::Playing { board } => { board },
            GameState::Finished { .. } => {
                return false;
            }
        };
        match msg {
            Msg::SelectCard { card } => {
                // the value has changed so we need to
                // re-render for it to appear on the page
                log::info!("Select card: {:?}",  card);
                self.selected_card = Some(card);
                true
            }
            Msg::SelectCell { x, y } => {
                log::info!("Select cell: {:?}", (x, y));
                let point = Point { x: x as i8, y: y as i8 };
                let src = match self.selected_square {
                    None => {
                        self.selected_square = Some(point);
                        return false;
                    }
                    Some(src) => src,
                };
                let card = match self.selected_card {
                    None => {
                        log::error!("No card selected");
                        return false;
                    }
                    Some(card) => card,
                };
                let game_move = Move::Move { card, src, dst: point };
                let new_state = match board.make_move(game_move) {
                    Ok(new_state) => new_state,
                    Err(err) => {
                        log::error!("Illegal move: {}", err);
                        self.selected_square = None;
                        return true;
                    }
                };
                self.state = new_state;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    //noinspection RsTypeCheck
    fn view(&self) -> Html {
        let board = match &self.state {
            GameState::Playing { board } => board,
            GameState::Finished { winner } => {
                return html! {
                    <div>{"Game Over"}</div>
                };
            }
        };
        let grid = board.to_grid();
        let render_row = |(y, row): (usize, &[GameSquare; 5])| -> VNode {
            let cell = |(x, square): (usize, &GameSquare) | {
                let callback = self.link.callback(move |_| Msg::SelectCell { x, y });
                square.render(callback)
            };
            let inner: Vec<Html> = row
                .iter()
                .enumerate()
                .map(cell)
                .collect();
            html! {
                <div class="row">{inner}</div>
            }
        };
        let grid: Vec<Html> = grid.iter().enumerate().map(render_row).collect();
        let render_card = |&card| {
            let onclick = self.link.callback(move |_| Msg::SelectCard { card });
            html! {
                <div onclick=onclick class="card">{card}</div>
            }
        };
        html! {
            <div>
                <div class="cards">
                    { board.blue_hand.iter().map(render_card).collect::<Html>() }
                </div>
                <div class="grid-outer">
                    <div class="grid">
                        { grid }
                    </div>
                </div>
                <div class="cards">
                    { board.red_hand.iter().map(render_card).collect::<Html>() }
                </div>
            </div>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Model>();
}