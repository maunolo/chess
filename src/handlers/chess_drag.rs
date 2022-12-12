use yew::{DragEvent as YewDragEvent};
use web_sys::{DragEvent, Document, NodeList, Element};
use log::debug;
use wasm_bindgen::JsCast;

struct JsDragEvent(DragEvent);

impl From::<YewDragEvent> for JsDragEvent {
  fn from(event: YewDragEvent) -> JsDragEvent {
    JsDragEvent(JsCast::dyn_ref::<DragEvent>(&event).unwrap().to_owned())
  }
}

struct ElementVec(Vec<Element>);

impl From::<NodeList> for ElementVec {
  fn from(list: NodeList) -> ElementVec {
    let range = 0..list.length();

    ElementVec(range.map(|i| {
      let node = list.item(i).unwrap();
      JsCast::dyn_ref::<Element>(&node).unwrap().to_owned()
    }).collect())
  }
}

fn document() -> Document {
  web_sys::window().unwrap().document().unwrap()
}

fn dragzones() -> Vec<Element> {
  ElementVec::from(document().query_selector_all(".dropzone").unwrap()).0
}

fn pieces() -> Vec<Element> {
  ElementVec::from(document().query_selector_all(".piece").unwrap()).0
}

fn current_piece() -> Option<Element> {
  document().query_selector(".dragging").unwrap()
}

fn event_target_elem(event: &YewDragEvent) -> Element {
  let target = event.target().unwrap();
  JsCast::dyn_ref::<Element>(&target).unwrap().to_owned()
}

pub fn drag(event: YewDragEvent) {
  let event = JsDragEvent::from(event).0;
  log::debug!("mov X: {:?}", event.movement_x());
  log::debug!("mov Y:{:?}", event.movement_y());
}

pub fn dragstart(event: YewDragEvent) {
  let piece = event_target_elem(&event);
  piece.set_class_name(format!("{} dragging", piece.class_name()).as_str());

  let event = JsDragEvent::from(event).0;
  let data_tranfer = event.data_transfer().unwrap();
  let blank = document().create_element("div").unwrap();
  data_tranfer.set_effect_allowed("grabbing");
  data_tranfer.set_drag_image(&blank, 0, 0);

  // let drag_zones = drag_zones();
  // for i in 0..drag_zones.length() {
  //   let dropzone = drag_zones.item(i).unwrap();
  //   let dropzone_element = JsCast::dyn_ref::<Element>(&dropzone).unwrap();
  //   dropzone_element.set_class_name("dropzone highlight");
  // }
}

pub fn dragend(event: YewDragEvent) {
  let piece = event_target_elem(&event);
  piece.set_class_name("piece");

  // let drag_zones = drag_zones();
  // for i in 0..drag_zones.length() {
  //   let dropzone = drag_zones.item(i).unwrap();
  //   let element = JsCast::dyn_ref::<Element>(&dropzone).unwrap().to_owned();
  //   element.set_class_name("dropzone");
  // }
}

pub fn dragenter(event: YewDragEvent) {

}

pub fn dragover(event: YewDragEvent) {
  event.prevent_default();
}

pub fn dragleave(event: YewDragEvent) {
  
}

pub fn dragdrop(event: YewDragEvent) {
  let elem = event_target_elem(&event);
  let piece = current_piece();
  if piece.is_some() {
    let piece = piece.unwrap();

    if elem.class_name().contains("dropzone") {
      let old_piece = elem.query_selector(".piece").unwrap();
      if old_piece.is_some() { old_piece.unwrap().remove(); }
      elem.append_child(&piece).unwrap();
    } else if elem.class_name().contains("piece") {
      let dropzone = elem.parent_node().unwrap();
      let dropzone = JsCast::dyn_ref::<Element>(&dropzone).unwrap();
      elem.remove();
      dropzone.append_child(&piece).unwrap();
    }
  }
  // let dropzone = event_target_elem(event);
  // let old_piece = dropzone.query_selector(".piece").unwrap();
  // if old_piece.is_some() {
  //   old_piece.unwrap().remove();
  // }
}