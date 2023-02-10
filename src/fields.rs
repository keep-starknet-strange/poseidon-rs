use core::clone::Clone;
use core::convert::{AsMut, AsRef};
use core::marker::Copy;
// use core::cmp::PartialEq;
use core::default::Default;
// use core::iter::IntoIterator;
// use core::ops::{Add, Mul};

pub mod arithmetic;
pub mod prime;

pub trait Zero: Sized {
    fn zero() -> Self;
    fn set_zero(&mut self) {
        *self = Self::zero();
    }
    fn is_zero(&self) -> bool;
}

pub trait One: Sized {
    fn one() -> Self;
    fn set_one(&mut self) {
        *self = Self::one();
    }
    fn is_one(&self) -> bool;
}

pub trait Field<const N: usize>:
    AsMut<[u64; N]> + AsRef<[u64; N]> + Copy + Clone + Default + From<[u64; N]> + Zero
{
    fn add_assign(&mut self, other: &Self);
    fn mul_assign(&mut self, other: &Self);
    fn pow_assign(&mut self, other: &Self);
}
