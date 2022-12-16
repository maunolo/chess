use yew::MouseEvent as YewMouseEvent;
use web_sys::{Element, DomRect};
// use log;
use crate::utils::class_list::ClassListExt;
use crate::utils::elements;
use crate::utils::style::StyleExt;

fn translate_value(event: &YewMouseEvent, board_bounding: DomRect) -> String {
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

fn move_piece(piece: &Element, event: &YewMouseEvent) {
  let chess_board = elements::query_selector(".chessboard").unwrap();

  if !piece.parent_element().unwrap().class_list_include("chessboard") {
    piece.remove();
    chess_board.append_child(&piece).unwrap();
  }

  let board_bounding = chess_board.get_bounding_client_rect();
  let translate_value = translate_value(event, board_bounding);

  piece.style_set("transform", &translate_value);
}

// Only works in the browser

pub fn mousemove(event: YewMouseEvent) {
  if let Some(piece) = elements::query_selector(".dragging") {
    move_piece(&piece, &event);
  }
}

pub fn mousedown(event: YewMouseEvent) {
  // Add dragging class to the piece
  let piece = elements::event_target_elem(&event);
  piece.class_list_add("dragging");

  // Highlight the square the piece is on
  if let Some(square) = piece.parent_element() {
    square.class_list_add("highlighted");
    square.class_list_add("hovered");
  }

  // Move the piece to client cursor position
  move_piece(&piece, &event);
}

pub fn mouseup(_event: YewMouseEvent) {
  let valid_move = true;

  // Remove dragging class from the piece
  if let Some(piece) = elements::query_selector(".dragging") {
    piece.remove();
    piece.style_set("transform", "");
    
    let hovered_square = elements::query_selector(".hovered");
    let highlighted_square = elements::query_selector(".highlighted");

    match valid_move {
      true => {
        // Move the piece to the hovered square
        if let Some(square) = hovered_square.clone() {
          square.append_child(&piece).unwrap();
        }
      },
      false => {
        // Move the piece back to the highlighted square
        if let Some(square) = highlighted_square.clone() {
          square.append_child(&piece).unwrap();
        }
      }
    }

    // Remove the hovered class from the square
    if let Some(square) = hovered_square {
      square.class_list_remove("hovered");
    }

    // Remove the highlighted class from the square
    if let Some(square) = highlighted_square {
      square.class_list_remove("highlighted");
    }

    piece.class_list_remove("dragging");
  }
}

pub fn mouseover(event: YewMouseEvent) {
  let target = elements::event_target_elem(&event);
  let square = if target.class_list_include("piece") {
    target.parent_element().unwrap()
  } else {
    target.clone()
  };

  if let Some(square) = elements::query_selector(".hovered") {
    square.class_list_remove("hovered");
  }

  if let Some(_piece) = elements::query_selector(".dragging") {
    square.class_list_add("hovered");
  }
}