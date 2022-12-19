use yew::MouseEvent as YewMouseEvent;
use web_sys::{Document, Element, DomRect};
use wasm_bindgen::JsCast;

use super::style::StyleExt;
use super::class_list::ClassListExt;

pub mod generate {
  use web_sys::Element;
  use crate::utils::element_pool::ElementPoolExt;

  pub fn selected_square(square: &str) -> Element {
    let elem = Element::find_or_create();
    elem.set_attribute("data-square", square).unwrap();
    elem.set_attribute("class", &format!("selected square-{}", square)).unwrap();
    elem
  }
}

pub fn document() -> Document {
  web_sys::window().unwrap().document().unwrap()
}

pub fn event_target_elem(event: &YewMouseEvent) -> Element {
  let target = event.target().unwrap();
  JsCast::dyn_ref::<Element>(&target).unwrap().to_owned()
}

pub fn query_selector(class: &str) -> Option<Element> {
  document().query_selector(class).unwrap()
}

pub fn add_to_board(elem: &Element) {
  let already_in_board = elem.parent_element().and_then(|e| Some(e.class_list_include("chessboard")));
  match already_in_board {
    Some(true) => {},
    Some(false) | None => {
      let chess_board = query_selector(".chessboard").unwrap();
      chess_board.append_child(elem).unwrap();
    }
  }
}

pub fn move_piece(piece: &Element, event: &YewMouseEvent) {
  let chess_board = piece.parent_element().unwrap();
  let translate_value = translate_value(event, &chess_board);

  piece.set_style("transform", &translate_value);
  piece.set_style("transition", "none");
}

fn translate_value(event: &YewMouseEvent, elem: &Element) -> String {
  let bounding = elem.get_bounding_client_rect();
  let (x, y) = mouse_position_in_bounding(event, &bounding);
  let translate_x = x - 50.0;
  let translate_y = y - 50.0;
  format!("translate({}%, {}%)", translate_x, translate_y)
}

pub fn mouse_position_in_bounding(event: &YewMouseEvent, bounding: &DomRect) -> (f64, f64) {
  let max_x = bounding.width();
  let max_y = bounding.height();

  let x = match event.client_x() as f64 - bounding.left() {
    x if x < 0.0 => 0.0,
    x if x > max_x => max_x,
    x => x
  };
  let y = match event.client_y() as f64 - bounding.top() {
    y if y < 0.0 => 0.0,
    y if y > max_y => max_y,
    y => y
  };

  let x_percent = x * 800.0 / max_x;
  let y_percent = y * 800.0 / max_y;

  (x_percent, y_percent)
}
