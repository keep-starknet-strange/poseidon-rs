use super::{
    arithmetic::{adc, add2, div_rem, ge, mac, sub2},
    Field, FpCfg, Montgomery,
};

use core::{
    clone::Clone,
    // debug_assert, unimplemented,
    cmp::{Eq, PartialEq},
    convert::{From, TryFrom},
    fmt,
    fmt::{Debug, Formatter},
    marker::{Copy, PhantomData},
    result::Result,
};

pub struct Fp<const N: usize, P: FpCfg<N>> {
    pub repr: [u64; N],
    pub phantom: PhantomData<P>,
}

impl<const N: usize, P: FpCfg<N>> Debug for Fp<N, P> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.as_ref().fmt(f)
    }
}

impl<const N: usize, P: FpCfg<N>> Clone for Fp<N, P> {
    fn clone(&self) -> Self {
        Self {
            repr: self.repr.clone(),
            phantom: PhantomData,
        }
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
        Self {
            repr: val,
            phantom: PhantomData,
        }
    }
}

impl<const N: usize, P: FpCfg<N>> TryFrom<&[u64]> for Fp<N, P> {
    type Error = &'static str;

    fn try_from(value: &[u64]) -> Result<Self, Self::Error> {
        if value.len() != N {
            Err("Passed wrong number of elements to represent a field element.")
        } else {
            let mut val = [0; N];
            for i in 0..N {
                val[i] = value[i];
            }
            let _ = div_rem(&mut val, &P::MOD, 0);
            Ok(Self {
                repr: val,
                phantom: PhantomData,
            })
        }
    }
}

impl<const N: usize, P: FpCfg<N>> From<u64> for Fp<N, P> {
    fn from(value: u64) -> Self {
        let mut val: [u64; N] = [0; N];
        val[0] = value;
        let mut res = Self::from(val);
        *res.from_int()
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

impl<const N: usize, P: FpCfg<N>> Field for Fp<N, P> {
    type BaseField = Self;

    const ZERO: Self = Self {
        repr: P::ZERO,
        phantom: PhantomData,
    };
    const ONE: Self = Self {
        repr: P::RADIX,
        phantom: PhantomData,
    };

    fn add_assign(&mut self, other: &Self) {
        let carry = add2(self.as_mut(), other.as_ref());
        if carry > 0 || ge(self.as_ref(), &P::MOD) {
            sub2(self.as_mut(), &P::MOD);
            // borrow == carry
        }
    }

    fn mul_assign(&mut self, other: &Self) {
        // Algorithm: Iterative Montgomery for Multi-Precision Felts
        let x = *self.clone().as_ref();
        let y = *other.as_ref();
        let res = self.as_mut();
        *res = [0; N];
        let mut t: u64;
        let mut carry: u128; // adding several u64 carries so we double it
        for i in 0..N {
            t = res[0];
            carry = mac(&mut t, x[i], y[0], 0) as u128;
            let q = P::NEG_INV.wrapping_mul(t);
            carry += mac(&mut t, q, P::MOD[0], 0) as u128;
            for j in 1..N {
                res[j - 1] = carry as u64;
                carry = carry >> u64::BITS;
                let sj = res[j];
                carry += adc(&mut res[j - 1], sj, 0) as u128;
                carry += mac(&mut res[j - 1], x[i], y[j], 0) as u128;
                carry += mac(&mut res[j - 1], q, P::MOD[j], 0) as u128;
            }
            res[N - 1] = carry as u64;
            // For starkware's prime, final_carry cannot be non-zero.
            // Since if self and other < MOD, then result < 2*MOD
        }
        if ge(res, &P::MOD) {
            sub2(res, &P::MOD);
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

impl<const N: usize, P: FpCfg<N>> Montgomery for Fp<N, P> {
    fn to_int(&mut self) -> &mut Self {
        let mut repr: [u64; N] = [0; N];
        repr[0] = 1;
        self.mul_assign(&Self::from(repr));
        self
    }

    fn from_int(&mut self) -> &mut Self {
        self.mul_assign(&Self::from(P::RADIX_SQ));
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fields::arithmetic::as_dbl_digit;

    pub struct P;
    impl FpCfg<2> for P {
        const MOD: [u64; 2] = [u64::MAX, (u64::MAX - 1) / 2]; // 2^127 - 1 is prime
        const RADIX: [u64; 2] = [2, 0];
        const RADIX_SQ: [u64; 2] = [4, 0];
        const NEG_INV: u64 = 1; // 2^64 | (1 * MOD + 1)
    }

    type Fp128 = Fp<2, P>;

    #[test]
    fn radix() {
        assert_eq!(
            (u128::MAX - as_dbl_digit(P::RADIX[0] - 1, P::RADIX[1]))
                % as_dbl_digit(P::MOD[0], P::MOD[1]),
            0
        );
    }

    #[test]
    fn inv() {
        assert_eq!(P::NEG_INV.wrapping_mul(P::MOD[0]).wrapping_add(1), 0);
    }

    #[test]
    fn from_u64_no_carry() {
        let input: u64 = 7;
        let expected = Fp128::from([14, 0]);
        assert_eq!(Fp128::from(input), expected);
    }

    #[test]
    fn from_u64_with_carry() {
        let input: u64 = u64::MAX;
        let expected = Fp128::from([u64::MAX - 1, 1]);
        assert_eq!(Fp128::from(input), expected);
    }

    #[test]
    fn mul_one_1() {
        let input = Fp128::from([2, 1]);
        let mut output = input.clone();
        output.mul_assign(&Fp128::ONE);
        assert_eq!(*input.as_ref(), *output.as_ref());
    }

    #[test]
    fn from_int_no_red() {
        let input = Fp128::from([2, 1]);
        let mut output = input.clone();
        output.from_int();
        assert_eq!(Fp128::from([4, 2]), output);
    }

    #[test]
    fn from_int_with_red() {
        let input = Fp128::from([u64::MAX - 3, (u64::MAX - 1) / 2]);
        let mut output = input.clone();
        output.from_int();
        assert_eq!(Fp128::from([u64::MAX - 6, (u64::MAX - 1) / 2]), output);
    }

    #[test]
    fn to_from_int() {
        let input = Fp128::from([2, 1]);
        let mut output = input.clone();
        output.from_int().to_int();
        assert_eq!(input, output);
    }

    #[test]
    fn from_no_division() {
        let input: [u64; 2] = [2, 1];
        let output = Fp128::from(input);
        assert_eq!(input, output.repr);
    }

    #[test]
    fn from_with_division() {
        let input: [u64; 2] = [1234, u64::MAX];
        let output = Fp128::from(input);
        assert_eq!([1235, u64::MAX / 2], output.repr);
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
        let mut input = Fp128::from([u64::MAX - 9, (u64::MAX - 1) / 2]);
        let other = Fp128::from([12, 0]);
        input.add_assign(&other);
        assert_eq!(*input.as_ref(), [3, 0]);
    }

    #[test]
    fn pow_assign_zero_exp() {
        let mut input = Fp128::from([4, 3]);
        input.pow_assign(0);
        assert_eq!(*input.as_ref(), *Fp128::ONE.as_ref());
    }

    #[test]
    fn mul_assign_1() {
        let mut input = Fp128::from([4, 3]);
        let other = Fp128::from([2, 0]);
        input.mul_assign(&other);
        assert_eq!(*input.as_ref(), [4, 3]);
    }

    #[test]
    fn mul_assign_2() {
        let mut input = Fp128::from([4, 1]);
        let other = Fp128::from([234, 0]);
        input.mul_assign(&other);
        assert_eq!(input, Fp128::from([468, 117]));
    }
}
