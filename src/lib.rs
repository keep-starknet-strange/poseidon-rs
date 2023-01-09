//! Poseidon Hash Functions for Ethereum
//!
//! Implements poseidon family of hash function over finite fields.
//! Hash functions depend on a set of parameters tightly linked to security.
//! Multiple sets of parameters are included in the crate. However, if users
//! wish so, they can provide their own set of parameters.
//!
//! Hash functions are exported through a C ABI in a shared library.
//! This allows the functions to be called in geth from golang.
//!
//! # Examples
//! Hash functions are named hash_<params>, where <params> is the name of the
//! set of parameters to be used. They take slices of field elements as input
//! and returns a vector of field elements representing the hash value. For
//! example:
//!
//! ```
//! use poseidon::hash_s128b;
//! use poseidon::parameters::s128b::F253;
//! let inputs = vec![F253::from(7), F253::from(54)];
//! let h = hash_s128b(&inputs);
//! ```


// Implementation is done for PrimeFields.
// Question remains of how to handle BinaryFields.
// Other fields are probably not useful at this point.

pub mod convert;
use convert::{felts_from_u8s, u8s_from_felts};

pub mod permutation;
pub use permutation::Poseidon;

pub mod parameters;
pub use parameters::s128b;
// add more parameters here.


pub fn hash_s128b(inputs: &[s128b::F253]) -> Vec<s128b::F253> {
    const M: usize = s128b::MDS.len();
    const N: usize = s128b::RK.len();
    Poseidon::<s128b::F253, M, N>::new(&s128b::PARAMS, &s128b::MDS, &s128b::RK)
        .hash(inputs)
        .expect("Testing")
}

// C-Interface for the hash function
#[no_mangle]
pub extern "C" fn c_hash_s128b(input: *const u8, input_len: usize, output: *mut u8, output_len: usize) -> usize {
    let input = unsafe {
        assert!(!input.is_null());
        std::slice::from_raw_parts(input, input_len)
    };
    let input = felts_from_u8s(&input);

    let result = hash_s128b(&input);
    let result = u8s_from_felts(&result);

    let count = result.len().min(output_len);
    // let src = result.as_ptr();
    let output = unsafe {
        assert!(!output.is_null());
        std::slice::from_raw_parts_mut(output, output_len)
    };
    output.copy_from_slice(&result);
    count
}
