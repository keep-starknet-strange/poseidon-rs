use ff::PrimeField;

pub fn load_felts<F>(arr: &[&str]) -> Vec<F>
where
    F: PrimeField
{
    let mut result = vec![F::ZERO; arr.len()];
    for i in 0..arr.len() {
        result[i] = F::from_str_vartime(arr[i]).unwrap();
    }
    result
}
