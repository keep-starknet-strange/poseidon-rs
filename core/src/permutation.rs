use crate::fields::Field;

use core::{
    cmp::{Eq, PartialEq},
    convert::{AsMut, AsRef},
    default::Default,
    fmt::Debug,
    marker::Copy,
};

pub struct Constants<const SIZE: usize, const N_ROUNDS: usize, GF: Field> {
    pub n_full_rounds: usize,
    pub sbox: u32,
    pub mds: [[GF; SIZE]; SIZE],
    pub rks: [[GF; SIZE]; N_ROUNDS],
}

pub trait Permutation {
    fn permute(&mut self);
}

pub trait Sponge<const RATE: usize, const SIZE: usize, GF: Field>: Copy + AsRef<[GF; SIZE]> + AsMut<[GF; SIZE]> + Permutation {
    fn absorb(&mut self, input: &[GF; RATE]) {
        let mut state = self.as_mut();
        for i in 0..RATE {
            state[i].add_assign(&input[i]);
        }
        self.permute();
    }

    fn squeeze(&mut self) -> [GF; RATE] {
        let state = self.as_ref();
        let mut result: [GF; RATE] = [GF::default(); RATE];
        for i in 0..RATE {
            result[i] = state[i];
        }
        self.permute();
        result
    }
}
