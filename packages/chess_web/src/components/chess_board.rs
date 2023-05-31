use yew::prelude::*;

use crate::entities::chess_board::ChessBoard as ChessBoardEntity;
use crate::components::board::BoardBackground;
use crate::components::coordinates::Coordinates;
use crate::handlers::mouse::{mousedown, mouseover};

#[derive(Properties, PartialEq)]
pub struct Props {
  pub chess_board: ChessBoardEntity,
}

#[function_component]
pub fn ChessBoard(props: &Props) -> Html {
  html! {
    <chess-board
      class={props.chess_board.css_class()}
      onmouseover={Callback::from(mouseover)}
    >
      <BoardBackground />
      <Coordinates is_white_view={props.chess_board.is_white_view} />
      {for props.chess_board.stones_and_positions().iter().map(|(position, stone)| {
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
  }
}