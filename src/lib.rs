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
