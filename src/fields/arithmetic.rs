/// Arithmetic for field elements underlying integer representation.
///
/// Field elements are represented by a non-negative integer.
/// In turn, unsigned integers are modeled as a fixed-length array of digits.
/// In our case, the Digit and DoubleDigit types are set to u64 and u128 respectively.
/// Note that DoubleDigit must have the capacity to hold two digits.

use core::debug_assert;

// ------------------------  Digits Operations  ---------------------------------

pub const BITS: u64 = 64;
pub const MAX: u64 = ((1u128 << BITS) - 1) as u64;

// Double 
#[inline(always)]
pub fn as_dbl_digit(lo: u64, hi: u64) -> u128 {
    ((hi as u128) << BITS) + lo as u128
}

#[inline(always)]
pub fn from_dbl_digit(val: u128) -> (u64, u64) {
    (val as u64, (val >> BITS) as u64)
}

/// Computes `a - (b + borrow)`, returning the result and the new borrow.
#[inline(always)]
pub fn sbb(a: &mut u64, b: u64, borrow: u8) -> u8 {
    let ret = (1u128 << BITS) + (*a as u128) - (b as u128) - (borrow as u128);
    *a = ret as u64;
    (ret >> BITS == 0) as u8
}

/// Computes `a += b + carry`, returns the new carry.
#[inline(always)]
pub fn adc(a: &mut u64, b: u64, carry: u64) -> u64 {
    let tmp = (*a as u128) + (b as u128) + (carry as u128);
    *a = tmp as u64;
    (tmp >> BITS) as u64
}

/// Computes `a += (b * c) + carry`, returning the new carry over.
#[inline(always)]
pub fn mac(a: &mut u64, b: u64, c: u64, carry: u64) -> u64 {
    let tmp = (*a as u128) + ((b as u128) * (c as u128)) + (carry as u128);
    *a = tmp as u64;
    (tmp >> BITS) as u64
}


// --------------------  Numbers Operations  --------------------

pub fn add2<const N: usize>(a: &mut [u64; N], b: &[u64; N]) -> u64 {
    let mut carry = 0u64;
    for (a, b) in a.iter_mut().zip(b) {
        carry = adc(a, *b, carry);
    }
    carry
}

/// Subtract a multiple.
/// a -= b * c
/// Returns a borrow (if a < b then borrow > 0).
pub fn sub_mul<const N: usize>(a: &mut [u64; N], b: &[u64; N], c: u64) -> u64 {
    // carry is between -big_digit::MAX and 0, so to avoid overflow we store
    // offset_carry = carry + big_digit::MAX
    let mut offset_carry = MAX;

    for (x, y) in a.iter_mut().zip(b) {
        // We want to calculate sum = x - y * c + carry.
        // sum >= -(big_digit::MAX * big_digit::MAX) - big_digit::MAX
        // sum <= big_digit::MAX
        // Offsetting sum by (big_digit::MAX << big_digit::BITS) puts it in DoubleBigDigit range.
        let offset_sum = as_dbl_digit(*x, MAX)
            - (MAX as u128) + (offset_carry as u128)
            - *y as u128 * c as u128;
        let (new_x, new_offset_carry) = from_dbl_digit(offset_sum);
        offset_carry = new_offset_carry;
        *x = new_x;
    }

    MAX - offset_carry  // returns borrow
}

pub fn div_rem_digit<const N: usize>(a: &mut [u64; N], b: u64) -> u64 {
    let mut rem = 0u64;
    rem += 1;
    // TODO: implement
    // for d in a.iter_mut().rev() {
        // let lhs = (hi as u128) << 64 + (lo as u128);
        // let rhs = divisor as u128;
        // *d = (lhs / rhs) as u64;
        // rem = (lhs % rhs) as u64;
    // }
    rem
}

pub fn div_rem<const N: usize>(a: &mut [u64; N], b: &[u64; N], c: u64) -> u64 {
    // Assumes quotient has at most a single digit.
    // Put otherwise, b's leading digit is nonzero.
    // This is the case for the modulus.
    // Also, c <= b[N-1]
    let (a0, a1, a2) = ((c as u128), (a[N-1] as u128), (a[N-2] as u128));
    let (b0, b1) = ((b[N-1] as u128), (b[N-2] as u128));
    debug_assert!(a0 <= b0);

    // Initial guess: a1 + a0*B = q * b0 + r
    let tmp =  as_dbl_digit(a1 as u64, a0 as u64);
    let (mut q, mut r) = if a0 < b0 {
        ((tmp / b0) as u64, (tmp % b0))
    } else { 
        debug_assert!(a0 == b0);
        (MAX, a0 + a1)
    };

    // r < b1 + b0*B, but it could be negative, in which case q is too large.
    // This happens if r*B + a2 < q*b1, so we adjust our guess.
    while r <= MAX as u128 && ((r << BITS) + a2) < (q as u128) * b1
    {
        q -= 1;
        r += b0;
    }
    
    // q is now either the correct quotient digit, or in rare cases 1 too large.
    // Subtract q from a. This may overflow, in which case we will have to correct.
    let mut borrow = sub_mul::<N>(a, &b, q);
    if borrow > a0 as u64 {
        // q is too large. We need to add back one multiple of b.
        q -= 1;
        borrow -= add2::<N>(a, &b);
    }
    // The top digit of a, stored in a0, has now been zeroed.
    debug_assert!((borrow as u128) == a0);
    q
}

// a += b*c for c a digit
pub fn mac_digit_with_carry<const N: usize>(a: &mut [u64; N], b: &[u64; N], c: u64) -> u64 {
    if c == 0 {
        return 0;
    }
    let mut carry = 0;
    for (a, &b) in a.iter_mut().zip(b) {
        carry = mac(a, b, c, carry);
    }
    carry
}

pub fn mac_digit<const N: usize>(a: &mut [u64; N], b: &[u64; N], c: u64) {
    let _: u64 = mac_digit_with_carry(a, b, c);
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn as_dbl_digit_1() {
        let expected: u128 = (1 << 65) + 45;
        assert_eq!(as_dbl_digit(45, 2), expected);
    }

    #[test]
    fn from_dbl_digit_1() {
        let val: u128 = (1 << 74) + 12;
        assert_eq!(from_dbl_digit(val), (12, 1024));
    }

    #[test]
    fn dbl_digit_as_from() {
        let (lo, hi) : (u64, u64) = (MAX, MAX);
        assert_eq!(from_dbl_digit(as_dbl_digit(lo, hi)), (lo, hi));
    }

    #[test]
    fn dbl_digit_from_as() {
        let val: u128 = (1 << 100) + 89;
        let (lo, hi) = from_dbl_digit(val);
        assert_eq!(as_dbl_digit(lo, hi), val);
    }

    fn sbb_test(a: u64, b: u64, c: u8) -> (u128, u128) {
        let mut new_a = a;
        let new_c = sbb(&mut new_a, b, c);
        let before: u128 = as_dbl_digit(a, new_c as u64);
        let after: u128 = (new_a as u128) + (b as u128) + (c as u128);
        (before, after)
    }

    #[test]
    fn sbb_no_borrow() {
        let (before, after) = sbb_test(56, 34, 0);
        assert_eq!(before, after);
    }

    #[test]
    fn sbb_with_borrow() {
        let (before, after) = sbb_test(23, 33, 1);
        assert_eq!(before, after);
    }

    fn adc_test(a: u64, b: u64, c: u64) -> (u128, u128) {
        let mut new_a: u64 = a;
        let new_c = adc(&mut new_a, b, c);
        let res: u128 = as_dbl_digit(new_a, new_c) - (b as u128) - (c as u128);
        let before: u128 = (a as u128) + (b as u128) + (c as u128);
        let after: u128 = as_dbl_digit(new_a, new_c);
        (before, after)
    }

    #[test]
    fn adc_no_carry() {
        let (before, after) = adc_test(54, 32, 1);
        assert_eq!(before, after);
    }

    #[test]
    fn adc_with_carry() {
        let (before, after) = adc_test((1 << 63) + 3, 1 << 63, 0);
        assert_eq!(before, after);
    }

    fn mac_test(a: u64, b: u64, c: u64, d: u64) -> (u128, u128) {
        let mut new_a: u64 = a;
        let new_d = mac(&mut new_a, b, c, d);
        let before: u128 = (a as u128) + (b as u128) * (c as u128) + (d as u128);
        let after: u128 = as_dbl_digit(new_a, new_d);
        (before, after)
    }

    #[test]
    fn mac_no_carry() {
        let (before, after) = mac_test(12, 3, 5, 1);
        assert_eq!(before, after);
    }

    #[test]
    fn mac_with_carry() {
        let (before, after) = mac_test(10, 1 << 63, 32, 0);
        assert_eq!(before, after);
    }
}
