use ff::*;

pub struct Parameters<F> {
    field: F,
    power: u32,
    rate: u8,
    capacity: u8,
    output_size: u8,
    n_partial_rounds: u32,
    n_full_rounds: u32,
    mds_matrix: Vec<Vec<F>>,
    round_constants: Vec<Vec<F>>,
}

pub fn str_to_field<F>(mat: Vec<Vec<&'static str>>) -> Vec<Vec<F>>
where
    F: PrimeField
{
    let mut result: Vec<Vec<F>> = Vec::new();
    for i in 0..mat.len() {
        let mut row: Vec<F> = Vec::new();
        for j in 0..mat[i].len() {
            let val: F = F::from_str(mat[i][j]).unwrap();
            row.push(val);
        }
        result.push(row);
    }
    result
}
