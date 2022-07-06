mod attacks;
mod bitboard;
mod index;
mod pieces;
mod state;

use std::time::{Duration, SystemTime};

pub fn init_attack_tables() -> () {

}

pub fn initial_position_state() -> state::State {
    state::Fen::init().to_state()
}