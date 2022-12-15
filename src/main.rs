use yew::prelude::*;
use entities::board::Board;
use entities::position::Position;
use handlers::mouse::{mousemove, mouseup, mousedown, mouseover};

mod entities;
mod handlers;
mod utils;

fn row_str(y: i32) -> String {
    (8 - y).to_string()
}

fn col_str(x: i32) -> String {
    let buffer: [u8; 1] = [x as u8 + 97];
    std::str::from_utf8(&buffer).unwrap().to_string()
}

#[function_component]
fn App() -> Html {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    // let fen = "4k2r/6r1/8/8/8/8/3R4/R3K3 w Qk - 0 1";
    // let fen = "4k3/qqqqqqqq/8/8/8/8/QQQQQQQQ/4K3 w Qk - 0 1";
    let board = Board::new(fen);
    // The is_white_view flag is used to flip the board
    let is_white_view = false;

    html! {
        <div
            class="container"
            onmousemove={Callback::from(mousemove)}
            onmouseup={Callback::from(mouseup)}
        >
            <chess-board class="chessboard">
                <svg viewBox="0 0 100 100" class="coordinates">
                    {for (0..8).map(|i| {
                        let i = if is_white_view { i } else { 7 - i };
                        html! {
                            <text x="0.50" y={format!("{}", 12.5 * i as f64 + 2.25)} font-size="2">{row_str(i)}</text>
                        }
                    })}
                    {for (0..8).map(|i| {
                        let i = if is_white_view { i } else { 7 - i };
                        html! {
                            <text x={format!("{}", 12.5 * i as f64 + 10.75)} y="99.50" font-size="2">{col_str(i)}</text>
                        }
                    })}
                </svg>
                {for (0..64).map(|i| {
                    let x = i % 8;
                    let y = i / 8;
                    let position = Position::new(x, y, is_white_view);

                    let color = if (i + position.y) % 2 == 0 { "white" } else { "black" };

                    html! {
                        <div
                            class={format!("square dropzone {} {}", color, position.to_string())}
                            onmouseover={Callback::from(mouseover)}
                        >
                            {if let Some(stone) = board.get(&position) {
                                html! {
                                    <div
                                        class="piece"
                                        onmousedown={Callback::from(mousedown)}
                                        ondragstart={Callback::from(|e: DragEvent| e.prevent_default())}
                                        style={format!("background-image: url({});", stone.image_url)}
                                    >
                                    </div>
                                    
                                }
                            } else {html! {}}}
                        </div>
                    }
                })}
            </chess-board>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}