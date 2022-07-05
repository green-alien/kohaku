use std::fmt;
//use termion::{color as tcolor};

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum Color {
    Black,
    White,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum Type {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Piece(pub Type, pub Color);

impl Piece {
    pub fn to_char(&self) -> char {
        let arr = match self.1 {
            Color::Black => ['♚', '♟', '♛', '♜', '♝', '♞'],
            Color::White => ['♔', '♙', '♕', '♖', '♗', '♘'],
        };
        let ch = match self.0 {
            Type::King   => arr[0],
            Type::Pawn   => arr[1],
            Type::Queen  => arr[2],
            Type::Rook   => arr[3],
            Type::Bishop => arr[4],
            Type::Knight => arr[5],
        };
        ch
    }
    pub fn to_string(&self) -> String {
        format!("{}", self.to_char())
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
