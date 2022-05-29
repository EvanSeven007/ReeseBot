#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn opposite(&self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }

    /* Simple 1 - 1 map function from each type of color to a corresponding string */ 
    pub fn color_to_string(self) -> String {
        match self {
            Color::White => String::from("white"), 
            Color::Black => String::from("blue"), //We are using blue until we graduate from a CLI program
        }
    }
}