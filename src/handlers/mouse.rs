use yew::MouseEvent as YewMouseEvent;
use web_sys::{Element, DomRect};
// use log;
use crate::utils::class_list::{ClassList, ClassListExt};
use crate::utils::elements::{chess_board, current_piece, event_target_elem, selected_square, hovered_square};
use crate::utils::style::{StyleExt, Style};

impl StyleExt for Element {
  fn style_set(&self, key: &str, value: &str) {
    let mut style = Style::new(&self.get_attribute("style").unwrap_or(String::new()));
    style.set(key, value);
    self.set_attribute("style", &style.to_string()).unwrap();
  }
}

impl ClassListExt for Element {
  fn class_list_add(&self, class: &str) {
    let mut class_list = ClassList::new(&self.class_name());
    class_list.add(class);
    self.set_class_name(&class_list.class_name);
  }

  fn class_list_remove(&self, class: &str) {
    let mut class_list = ClassList::new(&self.class_name());
    class_list.remove(class);
    self.set_class_name(&class_list.class_name);
  }

  fn class_list_include(&self, class: &str) -> bool {
    let class_list = ClassList::new(&self.class_name());
    class_list.class_name.split(" ").any(|c| c == class)
  }
}

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
  let chess_board = chess_board();
  let board_bounding = chess_board.get_bounding_client_rect();
  let translate_value = translate_value(event, board_bounding);

  if let Some(piece) = current_piece() {
    if !piece.parent_element().unwrap().class_list_include("chessboard") {
      piece.remove();
      chess_board.append_child(&piece).unwrap();
    }
    piece.style_set("transform", &translate_value);
  }
}

pub fn mousedown(event: YewMouseEvent) {
  // Add dragging class to the piece
  let piece = event_target_elem(&event);
  piece.class_list_add("dragging");

  // Add selected class to the square
  if let Some(square) = selected_square() {
    square.class_list_remove("selected");
  }

  let square = piece.parent_element().unwrap();
  square.class_list_add("selected");
}

pub fn mouseup(_event: YewMouseEvent) {
  // Remove dragging class from the piece
  if let Some(piece) = current_piece() {
    piece.remove();
    if let Some(square) = hovered_square() {
      piece.style_set("transform", "");
      square.append_child(&piece).unwrap();
    }

    piece.class_list_remove("dragging");
  }

  // Remove selected class from the square
  if let Some(square) = selected_square() {
    square.class_list_remove("selected");
  }
}

pub fn mouseover(event: YewMouseEvent) {
  let target = event_target_elem(&event);
  let square = if target.class_list_include("piece") {
    target.parent_element().unwrap()
  } else {
    target.clone()
  };

  if let Some(square) = hovered_square() {
    square.class_list_remove("hovered");
  }

  if let Some(_piece) = current_piece() {
    square.class_list_add("hovered");
  }
}