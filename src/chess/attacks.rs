/* useful constants */
use crate::chess::bitboard::BitBoard;
use crate::chess::index::{Index, INDEX_LOOK_UP};

const UNIVERSE: BitBoard = BitBoard::new(0xffffffffffffffff);
const EMPTY:    BitBoard = BitBoard::new(0x0);

const NOT_A_FILE:  BitBoard = BitBoard::new(0xfefefefefefefefe);
const NOT_H_FILE:  BitBoard = BitBoard::new(0x7f7f7f7f7f7f7f7f);
const NOT_1_RANK:  BitBoard = BitBoard::new(0xffffffffffffff00);
const NOT_8_RANK:  BitBoard = BitBoard::new(0xffffffffffffff);

const NOT_AB_FILE: BitBoard = BitBoard::new(0xfcfcfcfcfcfcfcfc);
const NOT_GH_FILE: BitBoard = BitBoard::new(0x3f3f3f3f3f3f3f3f);
const NOT_12_RANK: BitBoard = BitBoard::new(0xffffffffffff0000);
const NOT_78_RANK: BitBoard = BitBoard::new(0xffffffffffff);

const A_FILE: BitBoard = BitBoard::new(0x0101010101010101);
const H_FILE: BitBoard = BitBoard::new(0x8080808080808080);
const RANK_1: BitBoard = BitBoard::new(0x00000000000000ff);
const RANK_8: BitBoard = BitBoard::new(0xff00000000000000);

const A1_GUARD: BitBoard = BitBoard::new(0x01010101010101ff);
const H1_GUARD: BitBoard = BitBoard::new(0x80808080808080ff);
const A8_GUARD: BitBoard = BitBoard::new(0xff01010101010101);
const H8_GUARD: BitBoard = BitBoard::new(0xff80808080808080);

pub struct Offsets(i32, i32, i32, i32);

pub const BISHOP_OFFSETS: Offsets = Offsets(9, 7, -7, -9);
pub const ROOK_OFFSETS:   Offsets = Offsets(8, 1, -1, -8);

pub struct Guards(BitBoard, BitBoard, BitBoard, BitBoard); 

pub const BISHOP_GUARDS: Guards = Guards(H8_GUARD, A8_GUARD, H1_GUARD, A1_GUARD);
pub const ROOK_GUARDS: Guards = Guards(RANK_8, RANK_1, H_FILE, A_FILE);

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
        let pos = BitBoard::new(1 << i);
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

pub fn bishop_mask(index: Index) -> BitBoard {
      ray_gun(index, EMPTY,  9, H8_GUARD)
    | ray_gun(index, EMPTY,  7, A8_GUARD)
    | ray_gun(index, EMPTY, -7, H1_GUARD)
    | ray_gun(index, EMPTY, -9, A1_GUARD)
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

pub fn queen_mask(index: Index) -> BitBoard { 
    bishop_mask(index) | rook_mask(index) 
}

/** raygun
 * produce sliding piece ray
    from index idx
    with respect to occupency occ
    with cardnal offset
    
 */
pub fn ray_gun(
    idx: Index,
    occ: BitBoard,
    offset: i32,
    guard: BitBoard
) -> BitBoard {
    let mut bb = EMPTY;
    let mut cur_idx = idx;
    for _ in 0..8 {
        cur_idx = match cur_idx.shift(offset) {
            Some(idx) => idx,
            None => return bb,
        };
        let bit_idx = cur_idx.to_bitboard();
        bb |= bit_idx;
        if occ & bit_idx != EMPTY 
        || guard & bit_idx != EMPTY 
        {break}
    }
    bb
}


pub fn gen_table(
    offsets: Offsets,
    mask_fn: fn(Index) -> BitBoard,
    guards: Guards
) -> Vec<Vec<BitBoard>> {
    let mut table: Vec<Vec<BitBoard>> = vec!(vec!(); 64);
    for idx in INDEX_LOOK_UP {
        let attack_mask = mask_fn(idx);//rook_mask(idx);
        let cr = attack_mask.carry_rippler();

        //println!("{}", cr.len());

        for blocker in cr.iter() {
            //println!("{}", *blocker);
            let attack_set = 
              ray_gun(idx, *blocker, offsets.0, guards.0)
            | ray_gun(idx, *blocker, offsets.1, guards.1)
            | ray_gun(idx, *blocker, offsets.2, guards.2)
            | ray_gun(idx, *blocker, offsets.3, guards.3);

            table[idx as usize].push(attack_set);
        }
    }
    for row in table.iter() {
        for set in row {
            println!("{}", *set)
        }
    }
    table
}