use crate::chess::bitboard::BitBoard;

/// little endian - row major enumeration of squares
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub enum Index {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,    
}

/// lookup table for conversion ops
pub const INDEX_LOOK_UP: [Index; 64] = [
    Index::A1, Index::B1, Index::C1, Index::D1, Index::E1, Index::F1, Index::G1, Index::H1,
    Index::A2, Index::B2, Index::C2, Index::D2, Index::E2, Index::F2, Index::G2, Index::H2,
    Index::A3, Index::B3, Index::C3, Index::D3, Index::E3, Index::F3, Index::G3, Index::H3,
    Index::A4, Index::B4, Index::C4, Index::D4, Index::E4, Index::F4, Index::G4, Index::H4,
    Index::A5, Index::B5, Index::C5, Index::D5, Index::E5, Index::F5, Index::G5, Index::H5,
    Index::A6, Index::B6, Index::C6, Index::D6, Index::E6, Index::F6, Index::G6, Index::H6,
    Index::A7, Index::B7, Index::C7, Index::D7, Index::E7, Index::F7, Index::G7, Index::H7,
    Index::A8, Index::B8, Index::C8, Index::D8, Index::E8, Index::F8, Index::G8, Index::H8,
];


impl Index {

    /// return a bitboard with the index set
    pub fn to_bitboard(&self) -> BitBoard {
        let i = *self as u32;
        BitBoard::new(1 << i)
    }

    /// return an index shifted by a given offset
    pub fn shift(&self, offset: i32) -> Option<Index> {
        let shifted_indx = (*self as i32) + offset;
        if shifted_indx < 0 || shifted_indx >= 64 {return None}
        Some(INDEX_LOOK_UP[shifted_indx as usize])
    }
}