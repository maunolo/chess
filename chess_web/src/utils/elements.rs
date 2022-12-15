use yew::MouseEvent as YewMouseEvent;
use web_sys::{Document, Element};
use wasm_bindgen::JsCast;

pub fn document() -> Document {
  web_sys::window().unwrap().document().unwrap()
}

pub fn event_target_elem(event: &YewMouseEvent) -> Element {
  let target = event.target().unwrap();
  JsCast::dyn_ref::<Element>(&target).unwrap().to_owned()
}

pub fn chess_board() -> Element {
  document().query_selector(".chessboard").unwrap().unwrap()
}

pub fn selected_square() -> Option<Element> {
  document().query_selector(".selected").unwrap()
}

pub fn hovered_square() -> Option<Element> {
  document().query_selector(".hovered").unwrap()
}

pub fn current_piece() -> Option<Element> {
  document().query_selector(".dragging").unwrap()
}