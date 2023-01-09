pub mod s128b;

// use std::fmt::Debug;
// use std::str::FromStr;
use ff::PrimeField;

pub struct Parameters
{
    pub power: u8,
    pub rate: usize,
    pub capacity: usize,
    pub output_size: usize,
    pub n_partial_rounds: u32,
    pub n_full_rounds: u32,
}

pub fn load_mds<F, const M: usize>(mat: &[[&'static str; M]; M]) -> [[F; M]; M]
where
    F: PrimeField
    // F: PrimeField + FromStr, <F as FromStr>::Err: Debug
{
    let mut result = [[F::ZERO; M]; M];
    for i in 0..M {
        for j in 0..M {
            result[i][j] = F::from_str_vartime(mat[i][j]).unwrap();
        }
    }
    result
}

pub fn load_rk<F, const M: usize, const N: usize>(mat: &[[&'static str; M]; N]) -> [[F; M]; N]
where
    F: PrimeField
    // F: PrimeField + FromStr, <F as FromStr>::Err: Debug
{
    let mut result = [[F::ZERO; M]; N];
    for i in 0..M {
        for j in 0..N {
            result[j][i] = F::from_str_vartime(mat[j][i]).unwrap();
        }
    }
    result
}
