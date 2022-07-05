/* useful constants */
use crate::chess::bitboard::BitBoard;
use crate::chess::index::Index;

const UNIVERSE: BitBoard = BitBoard::new(0xffffffffffffffff);
const EMPTY: BitBoard = BitBoard::new(0x0);
const NOT_A_FILE: BitBoard = BitBoard::new(0xfefefefefefefefe);
const NOT_H_FILE: BitBoard = BitBoard::new(0x7f7f7f7f7f7f7f7f);
const NOT_1_RANK: BitBoard = BitBoard::new(0xffffffffffffff00);
const NOT_8_RANK: BitBoard = BitBoard::new(0xffffffffffffff);
const NOT_AB_FILE: BitBoard = BitBoard::new(0xfcfcfcfcfcfcfcfc);
const NOT_GH_FILE: BitBoard = BitBoard::new(0x3f3f3f3f3f3f3f3f);
const NOT_12_RANK: BitBoard = BitBoard::new(0xffffffffffff0000);
const NOT_78_RANK: BitBoard = BitBoard::new(0xffffffffffff);

/// produce unobstructed king moves from a given index
pub fn king_mask(index: Index) -> BitBoard {
    // index as a u32
    let i = index as u32;

    // nice little closure to deal with wrapping
    let king_anti_wrap_mask = || {
        let mut mask = UNIVERSE;
        let pos = BitBoard::new(1 << i);
        if pos & NOT_1_RANK == EMPTY {
            mask &= NOT_8_RANK
        } else if pos & NOT_8_RANK == EMPTY {
            mask &= NOT_1_RANK
        }
        if pos & NOT_A_FILE == EMPTY {
            mask &= NOT_H_FILE
        } else if pos & NOT_H_FILE == EMPTY {
            mask &= NOT_A_FILE
        }
        mask.is()
    };

    // 0x8380000000000382
    // 8|1 1 0 0 0 0 0 1
    // 7|0 0 0 0 0 0 0 1
    // 6|0 0 0 0 0 0 0 0
    // 5|0 0 0 0 0 0 0 0
    // 4|0 0 0 0 0 0 0 0
    // 3|0 0 0 0 0 0 0 0
    // 2|1 1 0 0 0 0 0 0
    // 1|0 1 0 0 0 0 0 1
    //   a b c d e f g h
    let km = 0x8380000000000382u64;
    // rotate king move pattern to the approriate index
    // mask outer files and columns to prevent wrapping
    BitBoard::new(km.rotate_left(i) & king_anti_wrap_mask())
}

/// produce knight moves from a given index
// bad bois can jump so obstruction doesnt matter
// lookup form a simple table should be possible
pub fn knight_mask(index: Index) -> BitBoard {
    // index as u32
    let i = index as u32;

    //"
    let knight_anti_wrap_mask = || {
        let mut mask = UNIVERSE;
        let pos = index.to_bitboard();
        if pos & NOT_12_RANK == EMPTY {
            mask &= NOT_78_RANK
        } else if pos & NOT_78_RANK == EMPTY {
            mask &= NOT_12_RANK
        }
        if pos & NOT_AB_FILE == EMPTY {
            mask &= NOT_GH_FILE
        } else if pos & NOT_GH_FILE == EMPTY {
            mask &= NOT_AB_FILE
        }
        mask.is()
    };

    //     0x442800000028440
    // 8     0 0 1 0 0 0 0 0
    // 7     0 1 0 0 0 0 1 0
    // 6     0 0 0 0 0 0 0 1
    // 5     0 0 0 0 0 0 0 0
    // 4     0 0 0 0 0 0 0 0
    // 3     0 1 0 0 0 0 0 0
    // 2     0 0 1 0 0 0 0 1
    // 1     0 0 0 0 0 0 1 0
    //       a b c d e f g h
    let km = 0x442800000028440u64;

    //"
    BitBoard::new(km.rotate_left(i) & knight_anti_wrap_mask())
}

/// produce rook moves from a given index
pub fn rook_mask(index: Index) -> BitBoard {
    // "
    let i = index as u32;

    // columns and rows
    let column = 0x001010101010100u64;
    let row = 0x7eu64;
    // shift columen and row and negate their intersection
    BitBoard::new(column.rotate_left(i % 8) ^ row.rotate_left(i - (i % 8)))
}
