//use crate::debug::*;
//use crate::chess::index::Index;
use std::ops::{
    BitAnd, BitOr, BitXor, Shl, Shr, 
    BitAndAssign, BitOrAssign, BitXorAssign
};
use std::fmt;
use bitintr::Pext;

/// a little endian, row-major, turth table for a chess board
#[derive(PartialEq, Clone, Copy)]
pub struct BitBoard(u64);

// Attack set of unobstructed rook on E5 //                      
//     0x10101010ef101010                //     0x0              
// 8     0 0 0 0 1 0 0 0                 // 8     0 0 0 0 0 0 0 0 
// 7     0 0 0 0 1 0 0 0                 // 7     0 0 0 0 0 0 0 0 
// 6     0 0 0 0 1 0 0 0                 // 6     0 0 0 0 0 0 0 0 
// 5     0 0 0 0 1 0 0 0                 // 5     0 0 0 0 0 0 0 0 
// 4     1 1 1 1 0 1 1 1                 // 4     0 0 0 0 0 0 0 0 
// 3     0 0 0 0 1 0 0 0                 // 3     0 0 0 0 0 0 0 0 
// 2     0 0 0 0 1 0 0 0                 // 2     0 0 0 0 0 0 0 0 
// 1     0 0 0 0 1 0 0 0                 // 1     0 0 0 0 0 0 0 0 
//                                       //                      
//       a b c d e f g h                 //       a b c d e f g h

impl BitBoard {

    // initialize a new BitBoard
    pub const fn new(u: u64) -> BitBoard {
        Self(u)
    }
    
    pub const fn empty() -> BitBoard {
        Self(0)
    }

    // Contents of BitBoard
    pub fn is(self) -> u64 {
        self.0
    }

    pub fn trailing_zeros(self) -> u32 {
        self.0.trailing_zeros()
    }

    // shamelessly copied form people way smarter than me:
    // https://www.chessprogramming.org/Flipping_Mirroring_and_Rotating#Horizontal
    
    pub fn flip_vertical(&self) -> BitBoard {
        let mut x = self.0;
        let k1 = 0x00ff00ff00ff00ffu64;
        let k2 = 0x0000ffff0000ffffu64;
        x = ((x >>  8) & k1) | ((x & k1) <<  8);
        x = ((x >> 16) & k2) | ((x & k2) << 16);
        x = ( x >> 32)       | ( x       << 32);
        Self(x)
    }

    pub fn mirror_horizontal(&self) -> BitBoard {
        let mut x = self.0;
        let k1 = 0x5555555555555555u64;
        let k2 = 0x3333333333333333u64;
        let k4 = 0x0f0f0f0f0f0f0f0fu64;
        x = ((x >> 1) & k1) | ((x & k1) << 1);
        x = ((x >> 2) & k2) | ((x & k2) << 2);
        x = ((x >> 4) & k4) | ((x & k4) << 4);
        Self(x)
    }

    pub fn flip_diag_a1h8(&self) -> BitBoard{
        let mut x = self.0;
        let mut t;
        let k1 = 0x5500550055005500u64;
        let k2 = 0x3333000033330000u64;
        let k4 = 0x0f0f0f0f00000000u64;
        t  = k4 & (x ^ (x << 28));
        x ^=       t ^ (t >> 28) ;
        t  = k2 & (x ^ (x << 14));
        x ^=       t ^ (t >> 14) ;
        t  = k1 & (x ^ (x <<  7));
        x ^=       t ^ (t >>  7) ;
        Self(x)
    }

    pub fn clockwise(&self) -> BitBoard {
        self.flip_diag_a1h8().flip_vertical()
    }

    pub fn anti_clockwise(&self) -> BitBoard {
        self.flip_vertical().flip_diag_a1h8()
    }

    pub fn turn_pi_radians(&self) -> BitBoard {
        self.flip_vertical().mirror_horizontal()
    }

    pub fn pext(&self, mask: u64) -> u64 {
        self.0.pext(mask)
    }

    // https://www.chessprogramming.org/Traversing_Subsets_of_a_Set
    pub fn carry_rippler(b :BitBoard) -> Vec<BitBoard> {
        let d = b.is();
        let mut n = 0u64;
        let mut v = vec!();
        loop {
            v.push(Self::new(n));
            n = (n - d) & d;
            if n == 0 {break}
        }
        v
    }
}

/* implement bit operations for BitBoard */


// &
impl BitAnd for BitBoard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

// &=
impl BitAndAssign for BitBoard {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = Self(self.0 & rhs.0)
    }
}

// |
impl BitOr for BitBoard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

// |=
impl BitOrAssign for BitBoard {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = Self(self.0 | rhs.0)
    }
}

// ^
impl BitXor for BitBoard {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

// ^=
impl BitXorAssign for BitBoard {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = Self(self.0 ^ rhs.0)
    }
}

// <<
impl Shl for BitBoard {
    type Output = Self;

    fn shl(self, Self(rhs): Self) -> Self::Output {
        let Self(lhs) = self;
        Self(lhs << rhs)
    }
}

// >>
impl Shr for BitBoard {
    type Output = Self;
    
    fn shr(self, Self(rhs): Self) -> Self::Output {
        let Self(lhs) = self;
        Self(lhs >> rhs)
    }
}

// do it over and over and over and over and over and over and over and ...
impl Iterator for BitBoard {
    type Item = BitBoard;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {return None}
        let bit = Self(1<< self.trailing_zeros());
        *self ^= bit;
        Some(bit)
    }
}

// make me look pretty
impl fmt::Display for BitBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    
        let n = self.is();
        // hex reperesentation for consise reference
        let mut output = format!("    {:#x}\n", n);
        // print the board from white's pov
        //rows
        for i in 0..8 {
            let row = 8 - i;
            output = format!("{}{}     ", output, row);
            //columns
            for j in 0..8 {
                let b = get(*self, (7 - i) * 8 + j);
                output = format!("{}{} ", output, b)
            }
            // new line after each row
            output.push('\n')
        }
        // column letters
        output.push_str("\n      a b c d e f g h");
        writeln!(f, "{}", output)
    }
}

fn get(b: BitBoard, i: usize) -> usize {
    ((b.is() >> i) & 1) as usize
}