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

//
// ```
// use poseidon::hash_s128b;
// use poseidon::parameters::s128b::GF;
// let inputs = vec![GF::from(7), GF::from(54)];
// let h = hash_s128b(&inputs);
// ```

// Implementation is done for PrimeFields.
// Question remains of how to handle BinaryFields.
// Other fields are probably not useful at this point.
#![cfg_attr(
    any(target_arch = "wasm32", all(not(feature = "std"), not(test))),
    no_std
)]

#[cfg(all(not(feature = "std"), not(test)))]
pub mod panic;

pub mod fields;
pub mod permutation;
pub mod poseidon;
pub mod variants;

#[cfg(all(feature = "sw2", feature = "c_bind"))]
pub use variants::sw2::hash::c_hash_sw2;
