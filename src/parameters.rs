pub mod s128b;
mod starkware;
pub use starkware::sw2;
pub use starkware::sw3;
pub use starkware::sw4;
pub use starkware::sw8;

mod mina;
pub use mina::pallas;
pub use mina::vesta;


pub struct Parameters {
    pub power: u8,
    pub rate: usize,
    pub capacity: usize,
    pub output_size: usize,
    pub n_partial_rounds: usize,
    pub n_full_rounds: usize,
    pub mds_matrix: &'static [&'static str],
    pub round_constants: &'static [&'static str],
}
