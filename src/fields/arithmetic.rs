/// Arithmetic for field elements underlying integer representation.
///
/// Field elements are represented by a non-negative integer.
/// In turn, unsigned integers are modeled as a fixed-length array of digits.
/// In our case, the Digit and DoubleDigit types are set to u64 and u128 respectively.
/// Note that DoubleDigit must have the capacity to hold two digits.
use core::debug_assert;

// ------------------------  Digits Operations  ---------------------------------

pub const BITS: u32 = u64::BITS;
pub const MAX: u64 = u64::MAX;

/// Get double digit from 2 digits, low and high.
#[inline(always)]
pub fn as_dbl_digit(lo: u64, hi: u64) -> u128 {
    ((hi as u128) << BITS) + lo as u128
}

/// Get low and high digits respectively from a double digit.
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

/// Computes `a += b`, returning the last carry.
pub fn add2<const N: usize>(a: &mut [u64; N], b: &[u64; N]) -> u64 {
    let mut carry = 0u64;
    for (a, b) in a.iter_mut().zip(b) {
        carry = adc(a, *b, carry);
    }
    carry
}

/// Computes `a -= b * c` for c a digit, returning the borrow (if a < b then borrow > 0).
pub fn sub_mul<const N: usize>(a: &mut [u64; N], b: &[u64; N], c: u64) -> u64 {
    // -MAX <= carry <= 0, offsetting by MAX to avoid overflow
    let mut offset_carry = MAX;

    for (x, y) in a.iter_mut().zip(b) {
        // We want to calculate sum = x - y * c + carry.
        // - MAX^2 - MAX <= sum <= MAX.
        // Offsetting sum by (MAX << BITS) puts it in DoubleDigit range.
        let offset_sum =
            as_dbl_digit(*x, MAX) - (MAX as u128) + (offset_carry as u128) - *y as u128 * c as u128;
        let (new_x, new_offset_carry) = from_dbl_digit(offset_sum);
        offset_carry = new_offset_carry;
        *x = new_x;
    }
    MAX - offset_carry // borrow after offsetting
}

/// Computes `a %= b` with a possible small carry on a, returning the quotient.
/// Assumes the quotient is a digit, which is the case for b = MODULUS in a prime field.
// Implement the general division ??
pub fn div_rem<const N: usize>(a: &mut [u64; N], b: &[u64; N], c: u64) -> u64 {
    // Assumes quotient has at most a single digit.
    // Put otherwise, b's leading digit is nonzero.
    // This is the case for the modulus.
    // Also, c <= b[N-1]
    debug_assert!(N > 1);
    let (a0, a1, a2) = ((c as u128), (a[N - 1] as u128), (a[N - 2] as u128));
    let (b0, b1) = ((b[N - 1] as u128), (b[N - 2] as u128));
    debug_assert!(a0 <= b0);

    // Initial guess: a1 + a0*B = q * b0 + r
    let tmp = as_dbl_digit(a1 as u64, a0 as u64);
    let (mut q, mut r) = if a0 < b0 {
        (tmp / b0, (tmp % b0))
    } else {
        (MAX as u128, a0 + a1)
    };

    // r < b1 + b0*B, but it could be negative, in which case q is too large.
    // This happens if r*B + a2 < q*b1, so we adjust our guess.
    while r <= MAX as u128 && ((r << BITS) + a2) < q * b1 {
        q -= 1;
        r += b0;
    }

    // q is now either the correct quotient digit, or in rare cases 1 too large.
    // Subtract q from a. This may overflow, in which case we will have to correct.
    let mut borrow = sub_mul::<N>(a, &b, q as u64);
    if borrow > a0 as u64 {
        // q is too large. We need to add back one multiple of b.
        q -= 1;
        borrow -= add2::<N>(a, &b);
    }
    // The top digit of a, stored in a0, has now been zeroed.
    debug_assert!((borrow as u128) == a0);
    q as u64
}

/// Computes `a += b * c` for c a digit, returning the carry.
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
        let (lo, hi): (u64, u64) = (MAX, MAX);
        assert_eq!(from_dbl_digit(as_dbl_digit(lo, hi)), (lo, hi));
    }

    #[test]
    fn dbl_digit_from_as() {
        let val: u128 = (1 << 100) + 89;
        let (lo, hi) = from_dbl_digit(val);
        assert_eq!(as_dbl_digit(lo, hi), val);
    }

    fn sbb_invariants(a: u64, b: u64, c: u8) -> (u128, u128) {
        let mut new_a = a;
        let new_c = sbb(&mut new_a, b, c);
        let before: u128 = as_dbl_digit(a, new_c as u64);
        let after: u128 = (new_a as u128) + (b as u128) + (c as u128);
        (before, after)
    }

    #[test]
    fn sbb_no_borrow() {
        let (before, after) = sbb_invariants(56, 34, 0);
        assert_eq!(before, after);
    }

    #[test]
    fn sbb_with_borrow() {
        let (before, after) = sbb_invariants(23, 33, 1);
        assert_eq!(before, after);
    }

    fn adc_invariants(a: u64, b: u64, c: u64) -> (u128, u128) {
        let mut new_a: u64 = a;
        let new_c = adc(&mut new_a, b, c);
        let res: u128 = as_dbl_digit(new_a, new_c) - (b as u128) - (c as u128);
        let before: u128 = (a as u128) + (b as u128) + (c as u128);
        let after: u128 = as_dbl_digit(new_a, new_c);
        (before, after)
    }

    #[test]
    fn adc_no_carry() {
        let (before, after) = adc_invariants(54, 32, 1);
        assert_eq!(before, after);
    }

    #[test]
    fn adc_with_carry() {
        let (before, after) = adc_invariants((1 << 63) + 3, 1 << 63, 0);
        assert_eq!(before, after);
    }

    fn mac_invariants(a: u64, b: u64, c: u64, d: u64) -> (u128, u128) {
        let mut new_a: u64 = a;
        let new_d = mac(&mut new_a, b, c, d);
        let before: u128 = (a as u128) + (b as u128) * (c as u128) + (d as u128);
        let after: u128 = as_dbl_digit(new_a, new_d);
        (before, after)
    }

    #[test]
    fn mac_no_carry() {
        let (before, after) = mac_invariants(12, 3, 5, 1);
        assert_eq!(before, after);
    }

    #[test]
    fn mac_with_carry() {
        let (before, after) = mac_invariants(10, 1 << 63, 32, 0);
        assert_eq!(before, after);
    }

    #[test]
    fn add2_no_carry() {
        let mut a: [u64; 2] = [2, 3];
        let b: [u64; 2] = [3, 5];
        let expected: ([u64; 2], u64) = ([5, 8], 0);
        let carry = add2::<2>(&mut a, &b);
        assert_eq!((a, carry), expected);
    }

    #[test]
    fn add2_with_inner_carry() {
        let mut a: [u64; 2] = [u64::MAX, 3];
        let b: [u64; 2] = [3, 5];
        let expected: ([u64; 2], u64) = ([2, 9], 0);
        let carry = add2::<2>(&mut a, &b);
        assert_eq!((a, carry), expected);
    }

    #[test]
    fn add2_with_final_carry() {
        let mut a: [u64; 2] = [u64::MAX, u64::MAX];
        let b: [u64; 2] = [3, 5];
        let expected: ([u64; 2], u64) = ([2, 5], 1);
        let carry = add2::<2>(&mut a, &b);
        assert_eq!((a, carry), expected);
    }

    #[test]
    fn sub_mul_no_borrow() {
        let mut a: [u64; 2] = [456, 23];
        let b: [u64; 2] = [100, 0];
        let c: u64 = 3;
        let expected: ([u64; 2], u64) = ([156, 23], 0);
        let borrow = sub_mul(&mut a, &b, c);
        assert_eq!((a, borrow), expected);
    }

    #[test]
    fn sub_mul_with_last_borrow() {
        let mut a: [u64; 2] = [456, 22];
        let b: [u64; 2] = [100, 8];
        let c: u64 = 3;
        let expected: ([u64; 2], u64) = ([156, MAX - 1], 1);
        let borrow = sub_mul(&mut a, &b, c);
        assert_eq!((a, borrow), expected);
    }

    #[test]
    fn sub_mul_with_2_borrow() {
        let mut a: [u64; 2] = [256, 22];
        let b: [u64; 2] = [100, 8];
        let c: u64 = 3;
        let expected: ([u64; 2], u64) = ([MAX - 43, MAX - 2], 1);
        let borrow = sub_mul(&mut a, &b, c);
        assert_eq!((a, borrow), expected);
    }

    #[test]
    fn div_rem_smaller() {
        let mut a: [u64; 2] = [2, 3];
        let b: [u64; 2] = [1, 4];
        let c: u64 = 0;
        let expected: ([u64; 2], u64) = ([2, 3], 0);
        let quotient = div_rem(&mut a, &b, c);
        assert_eq!((a, quotient), expected);
    }

    #[test]
    fn div_rem_no_carry() {
        let mut a: [u64; 2] = [2, 3];
        let b: [u64; 2] = [1, 1];
        let c: u64 = 0;
        let expected: ([u64; 2], u64) = ([0, 1], 2);
        let quotient = div_rem(&mut a, &b, c);
        assert_eq!((a, quotient), expected);
    }

    #[test]
    fn div_rem_inner_carry() {
        let mut a: [u64; 2] = [1, 3];
        let b: [u64; 2] = [1, 1];
        let c: u64 = 0;
        let expected: ([u64; 2], u64) = ([MAX, 0], 2);
        let quotient = div_rem(&mut a, &b, c);
        assert_eq!((a, quotient), expected);
    }

    #[test]
    fn div_rem_final_carry_eq() {
        let mut a: [u64; 2] = [2, 0];
        let b: [u64; 2] = [1, 1];
        let c: u64 = 1;
        let expected: ([u64; 2], u64) = ([3, 0], MAX);
        let quotient = div_rem(&mut a, &b, c);
        assert_eq!((a, quotient), expected);
    }
}
