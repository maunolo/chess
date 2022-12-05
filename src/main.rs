use board::Board;
use board::position::Position;

#[path="../src/entities/board.rs"]
pub mod board;

use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let fen = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 1";
    // let fen = "4k2r/6r1/8/8/8/8/3R4/R3K3 w Qk - 0 1";
    let board = Board::new(fen);
    let is_white_view = true;

    html! {
        <div class="container">
            {for (0..64).map(|i| {
                let x = i % 8;
                let y = i / 8;
                let position = Position {
                    x: if is_white_view { x } else { 7 - x },
                    y: if is_white_view { y } else { 7 - y }
                };

                let buffer: [u8; 1] = [position.x as u8 + 97];
                let col_str = std::str::from_utf8(&buffer).unwrap();
                let row_str = (8 - position.y).to_string();

                let color = if (i + position.y) % 2 == 0 { "white" } else { "black" };
                let stone = board.get(&position);

                html! {
                    <div class={format!("box {}", color)}>
                        <div class="dropzone">
                            {if stone.is_some() {
                                let stone = stone.unwrap();
                                html! {
                                    <div draggable="true" class="piece">
                                        <img src={stone.image_url} alt={format!("{} {}", stone.color, stone.name)} />
                                    </div>
                                }
                            } else {html! {}}}
                            {if (is_white_view && position.x == 0) || (!is_white_view && position.x == 7) {
                                html! {
                                    <div class="row-label">
                                        {row_str}
                                    </div>
                                }
                            } else {html! {}}}
                            {if (is_white_view && position.y == 7) || (!is_white_view && position.y == 0) {
                                html! {
                                    <div class="col-label">
                                        {col_str}
                                    </div>
                                }
                            } else {html! {}}}
                        </div>
                    </div>
                }
            })}
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}