use core::{
    cmp::{Eq, PartialEq},
    default::Default,
    fmt::Debug,
    marker::Copy,
};
// use core::iter::IntoIterator;
// use core::ops::{Add, Mul};

pub mod arithmetic;
pub mod prime;

pub trait Field: Copy + Default + PartialEq + Eq + Debug {
    type BaseField;

    const ZERO: Self;
    const ONE: Self;

    fn add_assign(&mut self, other: &Self);
    fn mul_assign(&mut self, other: &Self);
    fn pow_assign(&mut self, exp: u32);
}

pub trait FpCfg<const N: usize> {
    const MOD: [u64; N];
    const RADIX: [u64; N]; // Montgomery Radix is 2^64N % MOD
    const RADIX_SQ: [u64; N]; // Radix Square % MOD
    const NEG_INV: u64; // -(MOD^-1) % 2^64
    const ZERO: [u64; N] = [0u64; N];
}

pub trait Montgomery: Field {
    fn to_int(&mut self) -> &mut Self;
    fn from_int(&mut self) -> &mut Self;
}
