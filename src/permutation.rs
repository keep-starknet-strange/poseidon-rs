// use std::str::FromStr;
use ff::*;
use crate::parameters::{Parameters, load_mds, load_rk};


pub struct Poseidon<Fr, const M: usize, const N: usize> {
    params: &'static Parameters,
    mds_matrix: [[Fr; M]; M],
    round_constants: [[Fr; M]; N],
}
impl<Fr, const M: usize, const N: usize> Poseidon<Fr, M, N>
where
    Fr: PrimeField
{
    pub fn new(params: &'static Parameters, mds: &[[&'static str; M]; M], rk: &[[&'static str; M]; N]) -> Self {
        Poseidon {
            params: params,
            mds_matrix: load_mds(mds),
            round_constants: load_rk(rk),
        }
    }

    fn ark(&self, state: &mut Vec<Fr>, round: u32) {
        let r = usize::try_from(round).unwrap();
        for i in 0..state.len() {
            state[i].add_assign(&self.round_constants[r][i]);
        }
    }

    fn sbox_full(&self, state: &mut Vec<Fr>) {
        for j in 0..state.len() {
            let aux = state[j].clone();
            for _ in 1..self.params.power {
                state[j].mul_assign(aux);
            }
        }
    }

    fn sbox_partial(&self, state: &mut Vec<Fr>) {
        let i = state.len() - 1;
        let aux = state[i].clone();
        for _ in 1..self.params.power {
            state[i].mul_assign(&aux);
        }
    }

    fn mix(&self, state: &mut Vec<Fr>) {
        let mut new_state = vec![Fr::ZERO; state.len()];
        for i in 0..state.len() {
            for j in 0..state.len() {
                let mut mij = self.mds_matrix[i][j];
                mij.mul_assign(&state[j]);
                new_state[i].add_assign(&mij);
            }
        }
        state.clone_from(&new_state);
    }

    pub fn permute(&self, state: &mut Vec<Fr>, input: &[Fr]) {
        // let n_full_rounds = params.n_full_rounds.clone();
        // let n_partial_rounds = params.n_partial_rounds[t - 2].clone();
    
        for i in 0..input.len() {
            state[i].add_assign(input[i]);
        }
        let rf = self.params.n_full_rounds / 2;
        let rp = self.params.n_partial_rounds;
    
        for i in 0..rf {
            self.ark(state, i);
            self.sbox_full(state);
            self.mix(state);
        }
        for i in rf..(rf + rp) {
            self.ark(state, i);
            self.sbox_partial(state);
            self.mix(state);
        }
        for i in (rf + rp)..(2*rf + rp) {
            self.ark(state, i);
            self.sbox_full(state);
            self.mix(state);
        }
    }

    pub fn hash(&self, inputs: &[Fr]) -> Result<Vec<Fr>, String>
    {
        if inputs.len() == 0 {
            return Err("Empty inputs".to_string());
        }
        let n_remaining = inputs.len() % self.params.rate;
        // Fixed length hash only. How to handle otherwise? Is padding safe?
        if n_remaining != 0 {
            return Err(format!("Input length {} must be a multiple of the hash rate {}", inputs.len(), self.params.rate).to_string());
        }
    
        let mut state = vec![Fr::ZERO; M];
        // let rate = usize::try_from(self.params.rate).unwrap();
        for i in (0..inputs.len()).step_by(self.params.rate) {
            self.permute(&mut state, &inputs[i..(self.params.rate + i)]);
        }
        state.truncate(self.params.output_size);
        Ok(state)
    }
}

#[cfg(test)]
mod test_permutation {
    use super::*;
    use crate::parameters::s128b;
    use ff::{PrimeField, Field};
    
    fn get_input() -> Vec<s128b::F253> {
        let b1: s128b::F253 = s128b::F253::from(7);
        let b2: s128b::F253 = s128b::F253::from(98);
        vec![b1.clone(), b2.clone()]
    }

    fn load_felts<F, const M: usize>(arr: &[&str; M]) -> Vec<F>
    where
        F: PrimeField
    {
        let mut result = vec![F::ZERO; M];
        for i in 0..M {
            result[i] = F::from_str_vartime(arr[i]).unwrap();
        }
        result
    }

    #[test]
    fn test_ark() {
        const M: usize = s128b::MDS.len();
        const N: usize = s128b::RK.len();
        let hash = Poseidon::<s128b::F253, M, N>::new(&s128b::PARAMS, &s128b::MDS, &s128b::RK);
        let input = get_input();
        let mut state = vec![s128b::F253::ZERO; M];
        for i in 0..input.len() {
            state[i] = input[i].clone();
        }
        hash.ark(&mut state, 0);
        let expected = [
            "10187801339791605336251748402605479409606566396373491958667041943798551150218",
            "8824452141556477327634835943439996420519135454314677708228322513850226510123",
            "5264468709835621148349527988912247104353814123939106227116596276180070073104",
        ];
        let expected = load_felts::<s128b::F253, M>(&expected);
        assert_eq!(state, expected);
    }

    #[test]
    fn test_sbox_full() {
        const M: usize = s128b::MDS.len();
        const N: usize = s128b::RK.len();
        let hash = Poseidon::<s128b::F253, M, N>::new(&s128b::PARAMS, &s128b::MDS, &s128b::RK);
        let state = [
            "10187801339791605336251748402605479409606566396373491958667041943798551150218",
            "8824452141556477327634835943439996420519135454314677708228322513850226510123",
            "5264468709835621148349527988912247104353814123939106227116596276180070073104",
        ];
        let mut state = load_felts::<s128b::F253, M>(&state);
        hash.sbox_full(&mut state);
        let expected = [
            "9033127700447853090229678702028773675793347128105171639302548972716183808266", 
            "12584005788907507820847858681541330081079761745009746063606627523756483557914", 
            "481502024243180892202073663390242313819723218601880119953632240938269076973",
        ];
        let expected = load_felts::<s128b::F253, M>(&expected);
        assert_eq!(state, expected);
    }

    #[test]
    fn test_sbox_partial() {
        const M: usize = s128b::MDS.len();
        const N: usize = s128b::RK.len();
        let hash = Poseidon::<s128b::F253, M, N>::new(&s128b::PARAMS, &s128b::MDS, &s128b::RK);
        let state = [
            "10187801339791605336251748402605479409606566396373491958667041943798551150218",
            "8824452141556477327634835943439996420519135454314677708228322513850226510123",
            "5264468709835621148349527988912247104353814123939106227116596276180070073104",
        ];
        let mut state = load_felts::<s128b::F253, M>(&state);
        hash.sbox_partial(&mut state);
        let expected = [
            "10187801339791605336251748402605479409606566396373491958667041943798551150218",
            "8824452141556477327634835943439996420519135454314677708228322513850226510123",
            "481502024243180892202073663390242313819723218601880119953632240938269076973",
        ];
        let expected = load_felts::<s128b::F253, M>(&expected);
        assert_eq!(state, expected);
    }

    #[test]
    fn test_mix() {
        const M: usize = s128b::MDS.len();
        const N: usize = s128b::RK.len();
        let hash = Poseidon::<s128b::F253, M, N>::new(&s128b::PARAMS, &s128b::MDS, &s128b::RK);
        let state = [
            "9033127700447853090229678702028773675793347128105171639302548972716183808266", 
            "12584005788907507820847858681541330081079761745009746063606627523756483557914", 
            "481502024243180892202073663390242313819723218601880119953632240938269076973",
        ];
        let mut state = load_felts::<s128b::F253, M>(&state);
        hash.mix(&mut state);
        let expected = [
            "13203118710027330771388479782000018409212326464085107030609226142628639238173",
            "5548559894030014093638382051049588462182080648170927883872275099718526588448",
            "13953418491398777380386807509733024828619099654861449366171745749260147070969",
        ];
        let expected = load_felts::<s128b::F253, M>(&expected);
        assert_eq!(state, expected);
    }

    #[test]
    fn test_permutation() {
        const M: usize = s128b::MDS.len();
        const N: usize = s128b::RK.len();
        let hash = Poseidon::<s128b::F253, M, N>::new(&s128b::PARAMS, &s128b::MDS, &s128b::RK);
        let input = get_input();
        let mut state = vec![s128b::F253::ZERO; M];
        hash.permute(&mut state, &input);
        let expected_str = [
            "11053447091811250430558990262025664436943237361628909971717799705027922243051",
            "13019049864140962728034369799908465523605511214062523575537207652022843673387",
            "1746872083923296042231738669780413133264403148587998064386629453972513813613",
        ];
        let mut expected = vec![s128b::F253::ZERO; M];
        for i in 0..M {
            expected[i] = s128b::F253::from_str_vartime(expected_str[i]).unwrap();
        }
        assert_eq!(state, expected);
    }
}
