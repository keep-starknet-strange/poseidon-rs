use super::{
    Zero,
    Field,
    arithmetic::{
        adc,
        div_rem,
    },
};

use core::marker::{Copy, PhantomData};
use core::clone::Clone;
use core::unimplemented;

pub trait FpConfig<const N: usize>: Copy + Clone {
    const MODULUS: [u64; N];
}

pub trait PrimeField<P: FpConfig<N>, const N: usize>: Field<N>
{
    fn reduce(&mut self);
}

#[derive(Copy, Clone)]
pub struct Fp<P: FpConfig<N>, const N: usize> (pub [u64; N], pub PhantomData<P>);

impl<P: FpConfig<N>, const N: usize> PrimeField<P, N> for Fp<P, N> {
    fn reduce(&mut self) {
        div_rem::<N>(&mut self.0, &P::MODULUS, 0);
    }
}

impl<P: FpConfig<N>, const N: usize> Default for Fp<P, N> {
    fn default() -> Self {
        Self::zero()
    }
}

impl<P: FpConfig<N>, const N: usize> Zero for Fp<P, N> {
    fn zero() -> Self {
        Self([0u64; N], PhantomData)
    }

    fn is_zero(&self) -> bool {
        self.0 == Self::zero().0
    }
}

impl<P: FpConfig<N>, const N: usize> AsRef<[u64; N]> for Fp<P, N> {
    fn as_ref(&self) -> &[u64; N] {
        &self.0
    }
}

impl<P: FpConfig<N>, const N: usize> AsMut<[u64; N]> for Fp<P, N> {
    fn as_mut(&mut self) -> &mut [u64; N] {
        &mut self.0
    }
}

impl<P: FpConfig<N>, const N: usize> From<[u64; N]> for Fp<P, N> {
    fn from(value: [u64; N]) -> Self {
        Self(value, PhantomData)  // Missing Modulus
    }
}

impl<P: FpConfig<N>, const N: usize> Field<N> for Fp<P, N> {
    fn add_assign(&mut self, other: &Self) {
        let mut carry = 0u64;

        for (a, b) in self.as_mut().iter_mut().zip(other.as_ref()) {
            carry = adc(a, *b, carry);
        }

        // Modulus
    }

    fn mul_assign(&mut self, other: &Self) {
        unimplemented!();

        // Least-significant zeros have no effect on the output.
        // if let Some(&0) = other.0[0] {
            // if let Some(nz) = other.0.iter().position(|&d| d != 0) {
                // b = &b[nz..];
                // acc = &mut acc[nz..];
            // } else {
                // return;
            // }
        // }
        // if let Some(&0) = c.first() {
            // if let Some(nz) = c.iter().position(|&d| d != 0) {
                // c = &c[nz..];
                // acc = &mut acc[nz..];
            // } else {
                // return;
            // }
        // }

        // let acc = acc;
        // let (x, y) = if b.len() < c.len() { (b, c) } else { (c, b) };

        // Long multiplication:
        // for (i, xi) in x.iter().enumerate() {
        //     mac_digit(&mut acc[i..], y, *xi);
        // }
    }

//    if c == 0 {
//        return;
//    }
//
//    let mut carry = 0;
//    let (a_lo, a_hi) = acc.split_at_mut(b.len());
//
//    for (a, &b) in a_lo.iter_mut().zip(b) {
//        *a = mac_with_carry(*a, b, c, &mut carry);
//    }
//
//    let (carry_hi, carry_lo) = big_digit::from_doublebigdigit(carry);
//
//    let final_carry = if carry_hi == 0 {
//        __add2(a_hi, &[carry_lo])
//    } else {
//        __add2(a_hi, &[carry_hi, carry_lo])
//    };
//    assert_eq!(final_carry, 0, "carry overflow during multiplication!");

    fn pow_assign(&mut self, _other: &Self) {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::marker::Copy;
    use core::clone::Clone;

    #[derive(Copy, Clone)]
    pub struct P;
    impl FpConfig::<2> for P {
        const MODULUS: [u64; 2] = [5, 1];
    }

    #[test]
    fn test_add_assign() {
        let mut input = Fp::<P, 2>::from([2, 3]);
        let other = Fp::<P, 2>::from([2, 0]);
        input.add_assign(&other);
        assert_eq!(*input.as_ref(), [4, 3]);
    }

    #[test]
    fn test_add_assign_with_carry() {
        let mut input = Fp::<P, 2>::from([(2u128.pow(64) - 1) as u64, 3]);
        let other = Fp::<P, 2>::from([2, 0]);
        input.add_assign(&other);
        assert_eq!(*input.as_ref(), [1, 4]);
    }
}
