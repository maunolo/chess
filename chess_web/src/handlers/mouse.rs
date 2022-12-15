use yew::MouseEvent as YewMouseEvent;
use web_sys::DomRect;
// use log;
use crate::utils::class_list::ClassListExt;
use crate::utils::elements;
use crate::utils::style::StyleExt;

fn translate_value(event: YewMouseEvent, board_bounding: DomRect) -> String {
  let translate_x = match event.client_x() as f64 - board_bounding.left() {
    x if x < 0.0 => 0.0,
    x if x > 800.0 => 800.0,
    x => x
  } - 50.0;
  let translate_y = match event.client_y() as f64 - board_bounding.top() {
    y if y < 0.0 => 0.0,
    y if y > 800.0 => 800.0,
    y => y
  } - 50.0;
  format!("translate({}%, {}%)", translate_x, translate_y)
}

// Only works in the browser

pub fn mousemove(event: YewMouseEvent) {
  let chess_board = elements::chess_board();
  let board_bounding = chess_board.get_bounding_client_rect();
  let translate_value = translate_value(event, board_bounding);

  if let Some(piece) = elements::current_piece() {
    if !piece.parent_element().unwrap().class_list_include("chessboard") {
      piece.remove();
      chess_board.append_child(&piece).unwrap();
    }
    piece.style_set("transform", &translate_value);
  }
}

pub fn mousedown(event: YewMouseEvent) {
  // Add dragging class to the piece
  let piece = elements::event_target_elem(&event);
  piece.class_list_add("dragging");

  // Add selected class to the square
  if let Some(square) = elements::selected_square() {
    square.class_list_remove("selected");
  }

  let square = piece.parent_element().unwrap();
  square.class_list_add("selected");
}

pub fn mouseup(_event: YewMouseEvent) {
  // Remove dragging class from the piece
  if let Some(piece) = elements::current_piece() {
    piece.remove();
    if let Some(square) = elements::hovered_square() {
      piece.style_set("transform", "");
      square.append_child(&piece).unwrap();
      square.class_list_remove("hovered");
    }

    piece.class_list_remove("dragging");
  }

  // Remove selected class from the square
  if let Some(square) = elements::selected_square() {
    square.class_list_remove("selected");
  }
}

pub fn mouseover(event: YewMouseEvent) {
  let target = elements::event_target_elem(&event);
  let square = if target.class_list_include("piece") {
    target.parent_element().unwrap()
  } else {
    target.clone()
  };

  if let Some(square) = elements::hovered_square() {
    square.class_list_remove("hovered");
  }

  if let Some(_piece) = elements::current_piece() {
    square.class_list_add("hovered");
  }
}