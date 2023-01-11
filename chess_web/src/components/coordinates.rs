use yew::prelude::*;

fn row_str(y: i32) -> String {
  (8 - y).to_string()
}

fn col_str(x: i32) -> String {
  let buffer: [u8; 1] = [x as u8 + 97];
  std::str::from_utf8(&buffer).unwrap().to_string()
}

#[derive(Properties, PartialEq)]
pub struct Props {
  pub is_white_view: bool,
}

#[function_component]
pub fn Coordinates(props: &Props) -> Html {
  html! {
    <svg viewBox="0 0 100 100" class="coordinates">
      {for (0..8).map(|i| {
        let y = if props.is_white_view { i } else { 7 - i };
        html! {
          <text x="0.50" y={format!("{}", 12.5 * i as f64 + 2.25)} font-size="2">{row_str(y)}</text>
        }
      })}
      {for (0..8).map(|i| {
        let x = if props.is_white_view { i } else { 7 - i };
        html! {
          <text x={format!("{}", 12.5 * i as f64 + 10.75)} y="99.50" font-size="2">{col_str(x)}</text>
        }
      })}
    </svg>
  }
}