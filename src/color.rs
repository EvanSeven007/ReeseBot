#[derive(Clone, Copy)]
pub enum Color {
    White,
    Black,
}

/* Simple 1 - 1 map function from each type of color to a corresponding string */ 
pub fn color_to_string(color: Color) -> String {
    match color {
        Color::White => String::from("white"), 
        Color::Black => String::from("blue"), //We are using blue until we graduate from a CLI program
    }
}