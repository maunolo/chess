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

pub fn query_selector(class: &str) -> Option<Element> {
  document().query_selector(class).unwrap()
}