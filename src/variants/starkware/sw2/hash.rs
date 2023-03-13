use super::{new, GF};
use crate::{fields::Field, permutation::Sponge};

#[no_mangle]
pub extern "C" fn c_hash_sw2(
    input: *const u8,
    input_len: usize,
    output: *mut u8,
    output_len: usize,
) -> usize {
    assert!(input_len % 2 * 4 * 8 == 0);
    let mut poseidon = new([GF::ZERO; 3]);
    let input = unsafe {
        assert!(!input.is_null());
        crate::slice::from_raw_parts(input as *const u64, input_len / 8)
    };
    let mut input_block: [GF; 2] = [GF::default(); 2];
    for i in 0..(input.len() / 8) {
        for j in 0..2 {
            let idx = 8 * i + 4 * j;
            input_block[j] = GF::try_from(&input[idx..(idx + 4)]).unwrap();
        }
        poseidon.absorb(&input_block);
    }
    let result = poseidon.squeeze();
    let output = unsafe {
        assert!(!output.is_null());
        assert!(output_len >= 64);
        crate::slice::from_raw_parts_mut(output, output_len)
    };
    for i in 0..2 {
        let repr: [u64; 4] = result[i].repr;
        for j in 0..4 {
            let index = i * 32 + j * 8;
            let bytes = repr[j].to_le_bytes();
            output[index..(index + 8)].copy_from_slice(&bytes);
        }
    }
    64
}
