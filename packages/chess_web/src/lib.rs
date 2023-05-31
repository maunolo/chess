use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use yew::prelude::*;

use components::chess_board::ChessBoard;
use entities::chess_board::ChessBoard as ChessBoardEntity;
use handlers::mouse::{mousemove, mouseup};

mod components;
mod entities;
mod handlers;
mod utils;

#[function_component]
fn App() -> Html {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    // let fen = "4k2r/6r1/8/8/8/8/3R4/R3K3 w Qk - 0 1";
    // let fen = "4k3/qqqqqqqq/8/8/8/8/QQQQQQQQ/4K3 w Qk - 0 1";
    let mut chess_board = ChessBoardEntity::new(fen);

    // Flip the board
    chess_board.flip();

    html! {
      <div
        class="container"
        // ontouchmove={Callback::from(mousemove)}
        // ontouchup={Callback::from(mouseup)}
        onmousemove={Callback::from(mousemove)}
        onmouseup={Callback::from(mouseup)}
      >
        <ChessBoard chess_board={chess_board} />
      </div>
    }
}

#[global_allocator]
static ALLOC: std::alloc::System = std::alloc::System;

// This is the entry point for the web app
#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
    Ok(())
}
