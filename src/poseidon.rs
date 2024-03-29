use crate::{
    fields::{Field, Montgomery},
    permutation::{Constants, Permutation, Sponge},
};
use core::{clone::Clone, marker::Copy};

pub struct GenericPoseidon<'a, const RATE: usize, const SIZE: usize, const N_ROUNDS: usize, GF>
where
    GF: Field,
{
    pub state: [GF; SIZE],
    pub constants: &'a Constants<SIZE, N_ROUNDS, GF>,
}

impl<'a, const RATE: usize, const SIZE: usize, const N_ROUNDS: usize, GF> Clone
    for GenericPoseidon<'a, RATE, SIZE, N_ROUNDS, GF>
where
    GF: Field,
{
    fn clone(&self) -> Self {
        Self {
            state: self.state.clone(),
            constants: self.constants,
        }
    }
}

impl<'a, const RATE: usize, const SIZE: usize, const N_ROUNDS: usize, GF> Copy
    for GenericPoseidon<'a, RATE, SIZE, N_ROUNDS, GF>
where
    GF: Field,
{
}

impl<'a, const RATE: usize, const SIZE: usize, const N_ROUNDS: usize, GF> AsRef<[GF; SIZE]>
    for GenericPoseidon<'a, RATE, SIZE, N_ROUNDS, GF>
where
    GF: Field,
{
    fn as_ref(&self) -> &[GF; SIZE] {
        &self.state
    }
}

impl<'a, const RATE: usize, const SIZE: usize, const N_ROUNDS: usize, GF> AsMut<[GF; SIZE]>
    for GenericPoseidon<'a, RATE, SIZE, N_ROUNDS, GF>
where
    GF: Field,
{
    fn as_mut(&mut self) -> &mut [GF; SIZE] {
        &mut self.state
    }
}

impl<'a, const RATE: usize, const SIZE: usize, const N_ROUNDS: usize, GF>
    GenericPoseidon<'a, RATE, SIZE, N_ROUNDS, GF>
where
    GF: Field,
{
    pub fn ark(&mut self, round: usize) {
        let rks = self.constants.rks[round];
        let state = self.as_mut();
        for i in 0..SIZE {
            state[i].add_assign(&rks[i]);
        }
    }

    pub fn sbox_full(&mut self) {
        let sbox = self.constants.sbox;
        let state = self.as_mut();
        for i in 0..SIZE {
            state[i].pow_assign(sbox);
        }
    }

    pub fn sbox_partial(&mut self) {
        let sbox = self.constants.sbox;
        let state = self.as_mut();
        state[SIZE - 1].pow_assign(sbox);
    }

    pub fn mix(&mut self) {
        let state = self.as_ref();
        let mut new_state: [GF; SIZE] = [GF::default(); SIZE];
        for i in 0..SIZE {
            for j in 0..SIZE {
                let mut mij = self.constants.mds[i][j];
                mij.mul_assign(&state[j]);
                new_state[i].add_assign(&mij);
            }
        }
        let state = self.as_mut();
        for i in 0..SIZE {
            state[i] = new_state[i];
        }
    }
}

impl<'a, const RATE: usize, const SIZE: usize, const N_ROUNDS: usize, GF> Permutation
    for GenericPoseidon<'a, RATE, SIZE, N_ROUNDS, GF>
where
    GF: Field,
{
    fn permute(&mut self) {
        let rf = self.constants.n_full_rounds;
        let rp = N_ROUNDS - rf;
        let rf1 = rf / 2;
        let rf2 = rf1 + (rf % 2);

        for i in 0..rf1 {
            self.ark(i);
            self.sbox_full();
            self.mix();
        }
        for i in rf1..(rf1 + rp) {
            self.ark(i);
            self.sbox_partial();
            self.mix();
        }
        for i in (rf1 + rp)..(rf1 + rp + rf2) {
            self.ark(i);
            self.sbox_full();
            self.mix();
        }
    }
}

impl<'a, const RATE: usize, const SIZE: usize, const N_ROUNDS: usize, GF> Sponge<RATE, GF>
    for GenericPoseidon<'a, RATE, SIZE, N_ROUNDS, GF>
where
    GF: Montgomery,
{
    fn absorb(&mut self, input: &[GF; RATE]) {
        let state = self.as_mut();
        for i in 0..RATE {
            let mut el = input[i];
            el.from_int();
            state[i].add_assign(&el);
        }
        self.permute();
    }

    fn squeeze(&mut self) -> [GF; RATE] {
        let state = self.as_ref();
        let mut result: [GF; RATE] = [GF::default(); RATE];
        for i in 0..RATE {
            result[i] = state[i];
            result[i].to_int();
        }
        self.permute();
        result
    }

    fn hash(&self, inputs: &[GF]) -> [GF; RATE] {
        assert!(inputs.len() % RATE == 0);
        let mut sponge = self.clone();
        let mut input_block: [GF; RATE] = [GF::default(); RATE];
        for i in 0..(inputs.len() / RATE) {
            for j in 0..RATE {
                let idx: usize = RATE * i + j;
                input_block[j] = inputs[idx];
            }
            sponge.absorb(&input_block);
        }
        sponge.squeeze()
    }
}
