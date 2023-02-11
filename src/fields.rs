use core::{
    clone::Clone,
    cmp::{Eq, PartialEq},
    convert::{AsMut, AsRef},
    default::Default,
    fmt::Debug,
    marker::Copy,
};
// use core::iter::IntoIterator;
// use core::ops::{Add, Mul};

pub mod arithmetic;
pub mod prime;

pub trait Field<const N: usize>:
    AsMut<[u64; N]> + AsRef<[u64; N]> + Copy + Clone + Default + From<[u64; N]> + PartialEq + Eq + Debug
{
    const ZERO: Self;
    const ONE: Self;
    fn add_assign(&mut self, other: &Self);
    fn mul_assign(&mut self, other: &Self);
    fn pow_assign(&mut self, exp: u32);
}
