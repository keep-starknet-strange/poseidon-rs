use core::{
    clone::Clone,
    convert::{AsMut, AsRef},
    marker::Copy,
    default::Default,
    cmp::{PartialEq, Eq},
    fmt::Debug,
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
