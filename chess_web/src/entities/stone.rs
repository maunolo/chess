#[derive(Clone)]
pub struct Stone {
    pub c: String,
    pub color: String,
    pub name: String,
    pub image_url: String
}

impl Stone {
    pub fn from_char(c: char) -> Option<Self> {
        let c = c.to_string();
        let name = match c.to_uppercase().as_str() {
            "K" => "King",
            "Q" => "Queen",
            "R" => "Rook",
            "B" => "Bishop",
            "N" => "Knight",
            "P" => "Pawn",
            _ => "none"
        };
        let color = match c.as_str() {
            "p" | "r" | "n" | "b" | "q" | "k" => "Black",
            "P" | "R" | "N" | "B" | "Q" | "K" => "White",
            _ => "none"
        };
        let image_url = match c.as_str() {
            "p" => "/static/chess/Chess_pdt45.svg",
            "r" => "/static/chess/Chess_rdt45.svg",
            "n" => "/static/chess/Chess_ndt45.svg",
            "b" => "/static/chess/Chess_bdt45.svg",
            "q" => "/static/chess/Chess_qdt45.svg",
            "k" => "/static/chess/Chess_kdt45.svg",
            "P" => "/static/chess/Chess_plt45.svg",
            "R" => "/static/chess/Chess_rlt45.svg",
            "N" => "/static/chess/Chess_nlt45.svg",
            "B" => "/static/chess/Chess_blt45.svg",
            "Q" => "/static/chess/Chess_qlt45.svg",
            "K" => "/static/chess/Chess_klt45.svg",
            _ => "none"
        };

        if name == "none" || color == "none" || image_url == "none" {
            return None;
        }

        Some(Self {
            c,
            name: name.to_string(),
            color: color.to_string(),
            image_url: image_url.to_string()
        })
    }
}