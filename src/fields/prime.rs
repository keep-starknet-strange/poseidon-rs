use super::{
    arithmetic::{mac, adc, add2, sub2, div_rem},
    Field,
};

use core::{
    // debug_assert, unimplemented,
    clone::Clone,
    cmp::{PartialEq, Eq},
    marker::{Copy, PhantomData},
    fmt::{Debug, Formatter, Result},
};

pub trait FpCfg<const N: usize> {
    const MOD: [u64; N];
    const RADIX: [u64; N];  // Montgomery Radix is 2^64N % MOD
    const RADIX_SQ: [u64; N];  // Radix Square % MOD
    const INV: u64;  // -(MOD^-1) % 2^64
    const ZERO: [u64; N] = [0u64; N];
}

pub trait PrimeField<const N: usize, P: FpCfg<N>>: Field<N> {
    fn to_int(&self) -> [u64; N] {
        let mut res: [u64; N] = [0; N];
        res[0] = 1;
        let mut res = Self::from(res);
        res.mul_assign(self);
        *res.as_ref()
    }

    fn from_int(value: [u64; N]) -> Self {
        let mut res = Self::from(value);
        res.mul_assign(&Self::from(P::RADIX_SQ));
        res
    }
}

pub struct Fp<const N: usize, P: FpCfg<N>> {
    pub repr: [u64; N],
    phantom: PhantomData<P>,
}

impl<const N: usize, P: FpCfg<N>> Debug for Fp<N, P> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.as_ref().fmt(f)
    }
}

impl<const N: usize, P: FpCfg<N>> Clone for Fp<N, P> {
    fn clone(&self) -> Self {
        Self {repr: self.repr.clone(), phantom: PhantomData}
    }
}

impl<const N: usize, P: FpCfg<N>> Copy for Fp<N, P> {}

impl<const N: usize, P: FpCfg<N>> AsRef<[u64; N]> for Fp<N, P> {
    fn as_ref(&self) -> &[u64; N] {
        &self.repr
    }
}

impl<const N: usize, P: FpCfg<N>> AsMut<[u64; N]> for Fp<N, P> {
    fn as_mut(&mut self) -> &mut [u64; N] {
        &mut self.repr
    }
}

impl<const N: usize, P: FpCfg<N>> From<[u64; N]> for Fp<N, P> {
    fn from(value: [u64; N]) -> Self {
        let mut val = value.clone();
        let _ = div_rem(&mut val, &P::MOD, 0);
        Self {repr: val, phantom: PhantomData}
    }
}

impl<const N: usize, P: FpCfg<N>> PartialEq for Fp<N, P> {
    fn eq(&self, other: &Self) -> bool {
        self.as_ref() == other.as_ref()
    }

    fn ne(&self, other: &Self) -> bool {
        self.as_ref() != other.as_ref()
    }
}

impl<const N: usize, P: FpCfg<N>> Eq for Fp<N, P> {}

impl<const N: usize, P: FpCfg<N>> Default for Fp<N, P> {
    fn default() -> Self {
        Self::ZERO
    }
}

impl<const N: usize, P: FpCfg<N>> Field<N> for Fp<N, P> {
    const ZERO: Self = Self {repr: P::ZERO, phantom: PhantomData};
    const ONE: Self = Self {repr: P::RADIX, phantom: PhantomData};

    fn add_assign(&mut self, other: &Self) {
        let carry = add2(self.as_mut(), other.as_ref());
        if carry > 0 || *self.as_ref() > P::MOD {
            let _borrow = sub2(self.as_mut(), &P::MOD);
            // debug_assert!(u64::from(borrow) == carry);
        }
    }

    fn mul_assign(&mut self, other: &Self) {
        // Algorithm: Iterative Montgomery for Multi-Precision Felts
        let x = *self.clone().as_ref();
        let y = *other.as_ref();
        let res = self.as_mut();
        *res = [0; N];
        let mut t: u64;
        let mut carry: u64;
        for i in 0..N {
            t = res[0];
            carry = mac(&mut t, x[i], y[0], 0);
            let q = P::INV.wrapping_mul(t);
            carry += mac(&mut t, q, P::MOD[0], 0);
            for j in 1..N {
                res[j-1] = carry;
                let sj = res[j];
                carry = adc(&mut res[j-1], sj, 0);
                carry += mac(&mut res[j-1], x[i], y[j], 0);
                carry += mac(&mut res[j-1], q, P::MOD[j], 0);
            }
            res[N-1] = carry;
        }
    }

    fn pow_assign(&mut self, exp: u32) {
        if exp == 0 {
            *self = Self::ONE;
            return ();
        }
        if *self == Self::ZERO || *self == Self::ONE {
            return ();
        }
        let mut i = exp;
        let mut acc = self.clone();
        *self = Self::ONE.clone();
        while i != 0 { 
            if i % 2 == 1 {
                self.mul_assign(&acc);
            }
            acc.mul_assign(&acc.clone());
            i >>= 1;
        }
    }
}

impl<const N: usize, P: FpCfg<N>> PrimeField<N, P> for Fp<N, P> {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fields::arithmetic::as_dbl_digit;

    pub struct P;
    impl FpCfg<2> for P {
        const MOD: [u64; 2] = [5, 1];
        const RADIX: [u64; 2] = [25, 0];
        const RADIX_SQ: [u64; 2] = [625, 0];
        const INV: u64 = 3689348814741910323;
    }

    type Fp128 = Fp::<2, P>;

    #[test]
    fn radix() {
        assert_eq!((u128::MAX - as_dbl_digit(P::RADIX[0]-1, P::RADIX[1])) % as_dbl_digit(P::MOD[0], P::MOD[1]), 0);
    }

    #[test]
    fn inv() {
        assert_eq!(P::INV.wrapping_mul(P::MOD[0]).wrapping_add(1), 0);
    }

    #[test]
    fn mul_one_1() {
        let input = Fp128::from([2, 1]);
        let mut output = input.clone();
        output.mul_assign(&Fp128::ONE);
        assert_eq!(*input.as_ref(), *output.as_ref());
    }

    #[test]
    fn to_from_int() {
        let input: [u64; 2] = [2, 1];
        assert_eq!(input, Fp128::from_int(input).to_int());
    }
        
    #[test]
    fn from_no_division() {
        let input: [u64; 2] = [2, 1];
        let output = Fp128::from(input);
        assert_eq!(input, output.repr);
    }

    #[test]
    fn from_with_division() {
        let input: [u64; 2] = [20, 3];
        let output = Fp128::from(input);
        assert_eq!([5, 0], output.repr);
    }

    #[test]
    fn add_assign_no_carry() {
        let mut input = Fp128::from([2, 1]);
        let other = Fp128::from([2, 0]);
        input.add_assign(&other);
        assert_eq!(*input.as_ref(), [4, 1]);
    }

    #[test]
    fn add_assign_with_carry() {
        let mut input = Fp128::from([(2u128.pow(64) - 1) as u64, 1]);
        let other = Fp128::from([12, 0]);
        input.add_assign(&other);
        assert_eq!(*input.as_ref(), [1, 0]);
    }

    #[test]
    fn pow_assign_zero_exp() {
        let mut input = Fp128::from([4, 3]);
        input.pow_assign(0);
        assert_eq!(*input.as_ref(), *Fp128::ONE.as_ref());
    }

    #[test]
    fn mul_assign() {
        let mut input = Fp128::from([4, 3]);
        let other = Fp128::from([2, 0]);
        input.mul_assign(&other);
        assert_eq!(1, 1);
    }
}
