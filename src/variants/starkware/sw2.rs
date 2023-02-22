pub mod parameters;
pub use parameters::{new, Poseidon, CONSTANTS, GF};

#[cfg(feature = "c_bind")]
pub mod hash;

#[cfg(test)]
mod tests;
