mod attacks;
mod bitboard;
mod index;
mod pieces;
mod state;

use std::sync::Once;
use std::borrow::BorrowMut;

type Table = Vec<Vec<bitboard::BitBoard>>;
static mut ROOK_TABLE: Table = vec!();
static mut BISHOP_TABLE: Table = vec!();
static INIT: Once = Once::new();


pub fn init_attack_tables() -> () {
    INIT.call_once(|| {
        unsafe {
            *ROOK_TABLE.borrow_mut() 
                = attacks::gen_table(
                    attacks::ROOK_OFFSETS, 
                    attacks::rook_mask,
                    attacks::ROOK_GUARDS
                );
            //*BISHOP_TABLE.borrow_mut() 
            //    = attacks::gen_table(
            //        attacks::BISHOP_OFFSETS,
            //        attacks::bishop_mask,
            //        attacks::BISHOP_GUARDS
            //    );
        }
    });
    
}

pub fn bishop_lookup_table<'a>() -> &'a Table {
    unsafe { BISHOP_TABLE.as_ref() }
}
pub fn rook_lookup<'a>() -> &'a Table {
    unsafe { ROOK_TABLE.as_ref() }
}

pub fn initial_position_state() -> state::State {
    state::Fen::init().to_state()
}