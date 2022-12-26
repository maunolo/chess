use yew::prelude::*;

use entities::chess_board::ChessBoard as ChessBoardEntity;
use components::chess_board::ChessBoard;
use handlers::mouse::{mousemove, mouseup};

mod entities;
mod components;
mod handlers;
mod utils;

#[function_component]
fn App() -> Html {
  // let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
  // let fen = "4k2r/6r1/8/8/8/8/3R4/R3K3 w Qk - 0 1";
  let fen = "4k3/qqqqqqqq/8/8/8/8/QQQQQQQQ/4K3 w Qk - 0 1";
  let mut chess_board = ChessBoardEntity::new(fen);
  // let chess_board2 = chess_board.clone();
  // let chess_board3 = chess_board.clone();
  // let chess_board4 = chess_board.clone();

  // Flip the board
  chess_board.flip();

  html! {
    <div
      class="container"
      onmousemove={Callback::from(mousemove)}
      onmouseup={Callback::from(mouseup)}
    >
      <ChessBoard chess_board={chess_board} />
    </div>
  }
}

fn main() {
  wasm_logger::init(wasm_logger::Config::default());
  yew::Renderer::<App>::new().render();
}