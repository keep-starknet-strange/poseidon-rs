use crate::fields::Field;

pub struct Constants<const SIZE: usize, const N_ROUNDS: usize, GF: Field> {
    pub n_full_rounds: usize,
    pub sbox: u32,
    pub mds: [[GF; SIZE]; SIZE],
    pub rks: [[GF; SIZE]; N_ROUNDS],
}

pub trait Permutation {
    fn permute(&mut self);
}

pub trait Sponge<const RATE: usize, GF: Field>: Permutation {
    fn absorb(&mut self, input: &[GF; RATE]);
    fn squeeze(&mut self) -> [GF; RATE];
    fn hash(&self, inputs: &[GF]) -> [GF; RATE];
}
