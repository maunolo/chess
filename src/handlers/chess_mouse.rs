use yew::{MouseEvent as YewMouseEvent};
use web_sys::{MouseEvent, Document, NodeList, Element};
use log;
use wasm_bindgen::JsCast;

fn document() -> Document {
  web_sys::window().unwrap().document().unwrap()
}

fn event_target_elem(event: &YewMouseEvent) -> Element {
  let target = event.target().unwrap();
  JsCast::dyn_ref::<Element>(&target).unwrap().to_owned()
}

pub fn mousemove(event: YewMouseEvent) {
  let target = document().query_selector(".container").unwrap().unwrap();
  let box_bounding = target.get_bounding_client_rect();
  let x = (event.client_x() as f64) - box_bounding.left();
  let y = (event.client_y() as f64) - box_bounding.top();
  log::debug!("Left? : {} ; Top? : {}", x, y);
}

pub fn mousedown(event: YewMouseEvent) {
  log::debug!("mousedown");
  let piece = event_target_elem(&event);
  piece.set_class_name(format!("{} dragging", piece.class_name()).as_str());
  
}

pub fn mouseup(event: YewMouseEvent) {
  log::debug!("mouseup");
  let piece = event_target_elem(&event);
  piece.set_class_name("piece");
}