use crate::chess::bitboard::{BitBoard};

struct State {
    board: Board
    // meta
}

struct Board {
    black_pawns: BitBoard,
    black_knights: BitBoard,
    black_bishops: BitBoard,
    black_rooks: BitBoard,
    black_queens: BitBoard,
    black_king: BitBoard,
    w_pawns: BitBoard,
    w_knights: BitBoard,
    w_bishops: BitBoard,
    w_rooks: BitBoard,
    w_queens: BitBoard,
    w_king: BitBoard,

    black: BitBoard,
    white: BitBoard,
    occupency: BitBoard,
}

impl Board {
    pub fn new(f: Fen) -> Board {

        let [black_pawns, black_knights, black_bishops, black_rooks, black_queens, black_king,
            white_pawns, white_knights, white_bishops, white_rooks,  white_queens, white_king] = f.to_bitboards();

        let b = black_pawns | black_knights | black_bishops | black_rooks | black_queens | black_king;
        let w = white_pawns | white_knights | white_bishops | white_rooks | white_queens | white_king;
        let o = b | w;
        Board {
            black_pawns: black_pawns,
            black_knights: black_knights,
            black_bishops: black_bishops,
            black_rooks: black_rooks,
            black_queens: black_queens,
            black_king: black_king,
            w_pawns: white_pawns,
            w_knights: white_knights,
            w_bishops: white_bishops,
            w_rooks: white_rooks,
            w_queens: white_queens,
            w_king: white_king,
    
            black: b, 
            white: w,
            occupency: o, 
        }
    }
}

struct Fen(String);

impl Fen {
    pub fn to_bitboards(&self) -> [BitBoard; 12] {
        todo!()
        // iterate though self.0, pull out locations for each piece type
    }

    pub fn init() -> Fen {
        Fen(String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"))
    }
}