//! Poseidon Hash Functions for Ethereum
//!
//! Implements poseidon hash function over finite fields using the Hades
//! strategy. The hash function depends on several parameters, some tightly
//! linked to security and non-trivial to generate. This is why multiple sets
//! of parameters are implemented as part of the crate, parameters coming from
//! various use-cases. However, if the users wish so, they can provide their
//! own set of parameters.
//!
//! Ultimately, the goal of this library is to be included in ethereum's
//! execution layer such as Geth. The hash functions are therefore exported
//! through a C ABI in a shared library.
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
//
pub mod parameters;
pub mod permutation;
pub use permutation::Poseidon;
pub use parameters::s128b;

pub fn hash_s128b(inputs: &[s128b::F253]) -> Vec<s128b::F253> {
    const M: usize = s128b::MDS.len();
    const N: usize = s128b::RK.len();
    Poseidon::<s128b::F253, M, N>::new(&s128b::PARAMS, &s128b::MDS, &s128b::RK)
        .hash(inputs)
        .expect("Testing")
}

// Should add a C-Interface to call hash functions from shared_library.
