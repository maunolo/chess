use yew::prelude::*;

use entities::chess_board::ChessBoard;
use components::board::BoardBackground;
use components::coordinates::Coordinates;
use handlers::mouse::{mousemove, mouseup, mousedown, mouseover};

mod entities;
mod components;
mod handlers;
mod utils;

#[function_component]
fn App() -> Html {
  let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
  // let fen = "4k2r/6r1/8/8/8/8/3R4/R3K3 w Qk - 0 1";
  // let fen = "4k3/qqqqqqqq/8/8/8/8/QQQQQQQQ/4K3 w Qk - 0 1";
  let mut chess_board = ChessBoard::new(fen);

  // Flip the board
  // chess_board.flip();

  html! {
    <div
      class="container"
      onmousemove={Callback::from(mousemove)}
      onmouseup={Callback::from(mouseup)}
    >
      <chess-board
        class={chess_board.css_class()}
        onmouseover={Callback::from(mouseover)}
      >
        <BoardBackground />
        <Coordinates is_white_view={chess_board.is_white_view} />
        {for chess_board.stones_and_positions().iter().map(|(position, stone)| {
          html! {
            <div
              class={format!("piece {} {}", stone.image_class, position.css_class())}
              onmousedown={Callback::from(mousedown)}
              ondragstart={Callback::from(|e: DragEvent| e.prevent_default())}
              data-square={position.to_string()}
            ></div>
          }
        })}
      </chess-board>
    </div>
  }
}

fn main() {
  wasm_logger::init(wasm_logger::Config::default());
  yew::Renderer::<App>::new().render();
}