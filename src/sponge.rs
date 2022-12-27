use crate::parameters::Parameters;
use crate::permutation::permute;


pub fn hash<F>(inputs: &[F], params: &Parameters<F>) -> Result<F, String>
where
    F: PrimeField
{
    if inputs.len() == 0 || inputs.len() >= params.n_partial_rounds.len() - 1 {
        return Err("Wrong inputs length".to_string());
    }
    let n_remaining = inputs.len() % params.rate;
    // Fixed length hash only. How to handle otherwise? Is padding safe?
    if n_remaining != 0 {
        return Err("Input length must be a multiple of the hash size".to_string());
    }

    let mut state = vec![F::zero(); params.rate + params.capacity];
    for i in (0..inputs.len()).step_by(params.rate) {
        permute(&state, &inputs[(params.rate*i)..(params.rate*(i+1))], params);
    }
    Ok(state[0])
}


#[cfg(test)]
extern crate ff;
use ff::*;
use super::*;
use crate::parameters::S128b as S128b;

#[test]
fn test_hash() {
    let b0: S128b::F253 = S128b::F253::from_str("0").unwrap();
    let b1: S128b::F253 = S128b::F253::from_str("1").unwrap();
    let b2: S128b::F253 = S128b::F253::from_str("2").unwrap();
    let b3: S128b::F253 = S128b::F253::from_str("3").unwrap();
    let b4: S128b::F253 = S128b::F253::from_str("4").unwrap();
    let b5: S128b::F253 = S128b::F253::from_str("5").unwrap();
    let b6: S128b::F253 = S128b::F253::from_str("6").unwrap();

    let mut big_arr: Vec<S128b::F253> = Vec::new();
    big_arr.push(b1.clone());
    let h = hash(big_arr.clone(), S128b::params()).unwrap();
    assert_eq!(
        h.to_string(),
        "0x29176100eaa962bdc1fe6c654d6a3c130e96a4d1168b33848b897dc502820133" // "18586133768512220936620570745912940619677854269274689475585506675881198879027"
    );
}
