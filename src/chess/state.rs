use crate::chess::bitboard::BitBoard;
use crate::chess::pieces::Color;
use crate::chess::index::Index;

/// total game state
#[derive(Debug)]
pub struct State {
    // pysical
    pub board: Board, 
    // meta
    pub active_color: Color,
    pub castling_rights: String,
    pub en_passant: Option<Index>,
    pub half_move_count: u8,
    pub full_move_count: u8,
}

impl State {}

/// pysical game state
/// a piece wise representation of a chess board and it's pieces
#[derive(Debug)]
pub struct Board {
    pub black_pawns: BitBoard,
    pub black_knights: BitBoard,
    pub black_bishops: BitBoard,
    pub black_rooks: BitBoard,
    pub black_queens: BitBoard,
    pub black_king: BitBoard,
    pub white_pawns: BitBoard,
    pub white_knights: BitBoard,
    pub white_bishops: BitBoard,
    pub white_rooks: BitBoard,
    pub white_queens: BitBoard,
    pub white_king: BitBoard,

    pub black: BitBoard,
    pub white: BitBoard,
    pub occupency: BitBoard,
}

impl Board {
    pub fn from_fen(fen_board: String) -> Self {
        let mut idx = 0;
        let [mut bp, mut bn, mut bb, mut br, mut bq, mut bk,
             mut wp, mut wn, mut wb, mut wr, mut wq, mut wk] 
            = [0; 12];
        let fen_board = fen_board
            .rsplit("/")
            .fold(String::from(""), 
                |out, new| {[out, new.to_string()].concat()}
            );

        for ch in fen_board.chars() {
            match ch {
                'p' => bp |= 1 << idx,
                'n' => bn |= 1 << idx,
                'b' => bb |= 1 << idx,
                'r' => br |= 1 << idx,
                'q' => bq |= 1 << idx,
                'k' => bk |= 1 << idx,
                'P' => wp |= 1 << idx,
                'N' => wn |= 1 << idx,
                'B' => wb |= 1 << idx,
                'R' => wr |= 1 << idx,
                'Q' => wq |= 1 << idx,
                'K' => wk |= 1 << idx,
                '1'..='8' => idx += ch as u32 - '0' as u32 - 1,
                _ => unreachable!(),
            };
            idx += 1;
        }
        let b = bp | bn | bb | br | bq | bk;
        let w = wp | wn | wb | wr | wq | wk;
        let o = b | w;

        Board {
            black_pawns:   BitBoard::new(bp),
            black_knights: BitBoard::new(bn),
            black_bishops: BitBoard::new(bb),
            black_rooks:   BitBoard::new(br),
            black_queens:  BitBoard::new(bq),
            black_king:    BitBoard::new(bk),
            white_pawns:   BitBoard::new(wp),
            white_knights: BitBoard::new(wn),
            white_bishops: BitBoard::new(wb),
            white_rooks:   BitBoard::new(wr),
            white_queens:  BitBoard::new(wq),
            white_king:    BitBoard::new(wk),

            black:     BitBoard::new(b),
            white:     BitBoard::new(w),
            occupency: BitBoard::new(o),
        }
    }
}



/// Forsyth Edwards Notation
pub struct Fen(String);


impl Fen {
    pub fn to_state(&self) -> State {
        let mut split_fen = self.0.split(" ");
        let board_str = split_fen.next().unwrap();
        let board = Board::from_fen(board_str.to_string());        
        
        let ac_str = split_fen.next().unwrap();
        let active_color = match ac_str {
            "b" => Color::Black,
            "w" => Color::White,
            _ => unreachable!(),
        };

        let castling_rights = split_fen.next().unwrap().to_string();

        let ep_str = split_fen.next().unwrap();
        let en_passant = Index::from_str(ep_str);

        let hmc_str = split_fen.next().unwrap();
        let half_move_count = hmc_str.parse::<u8>().unwrap();

        let fmc_str = split_fen.next().unwrap();
        let full_move_count = fmc_str.parse::<u8>().unwrap();

        State {
            board: board,
            active_color: active_color,
            castling_rights: castling_rights,
            en_passant: en_passant,
            half_move_count: half_move_count,
            full_move_count: full_move_count,
        }
    }

    pub fn init() -> Fen {
        Fen(String::from(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        ))
    }
}
