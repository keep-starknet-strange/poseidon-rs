use crate::convert::felts_from_str;
use crate::parameters::Parameters;
use ff::PrimeField;
use crate::vec::Vec;
use crate::string::{String, ToString};

pub struct Poseidon<'a, GF> {
    params: &'a Parameters,
    mds_matrix: Vec<GF>,
    round_constants: Vec<GF>,
    offset: usize,
    state: Vec<GF>,
}

impl<'a, GF> Poseidon<'a, GF>
where
    GF: PrimeField,
{
    pub fn new(params: &'a Parameters) -> Self {
        Poseidon {
            params: params,
            mds_matrix: felts_from_str(params.mds_matrix),
            round_constants: felts_from_str(params.round_constants),
            offset: 0,
            state: vec![GF::ZERO; params.rate + params.capacity],
        }
    }

    fn ark(&mut self, round: usize) {
        let size = self.state.len();
        for i in 0..size {
            self.state[i].add_assign(&self.round_constants[round * size + i]);
        }
    }

    fn sbox_full(&mut self) {
        for j in 0..self.state.len() {
            let aux = self.state[j].clone();
            for _ in 1..self.params.power {
                self.state[j].mul_assign(aux);
            }
        }
    }

    fn sbox_partial(&mut self) {
        let i = self.state.len() - 1;
        let aux = self.state[i].clone();
        for _ in 1..self.params.power {
            self.state[i].mul_assign(&aux);
        }
    }

    fn mix(&mut self) {
        let size = self.state.len();
        let mut new_state = vec![GF::ZERO; size];
        for i in 0..size {
            for j in 0..size {
                let mut mij = self.mds_matrix[i * size + j];
                mij.mul_assign(&self.state[j]);
                new_state[i].add_assign(&mij);
            }
        }
        self.state.clone_from(&new_state);
    }

    fn permute(&mut self) {
        let rf = self.params.n_full_rounds / 2;
        let rp = self.params.n_partial_rounds;

        for i in 0..rf {
            self.ark(i);
            self.sbox_full();
            self.mix();
        }
        for i in rf..(rf + rp) {
            self.ark(i);
            self.sbox_partial();
            self.mix();
        }
        for i in (rf + rp)..(2 * rf + rp) {
            self.ark(i);
            self.sbox_full();
            self.mix();
        }
    }

    fn ensure_permuted(&mut self) {
        // offset should be <= rate, so really testing for equality
        if self.offset >= self.params.rate {
            self.permute();
            self.offset = 0;
        }
    }

    pub fn absorb(&mut self, input: &GF) {
        self.ensure_permuted();
        self.state[self.offset].add_assign(input);
        self.offset += 1;
    }

    pub fn squeeze(&mut self) -> GF {
        self.ensure_permuted();
        let result = self.state[self.offset];
        self.offset += 1;
        result
    }
}

pub fn hash<'a, GF>(inputs: &'a [GF], params: &'a Parameters) -> Result<Vec<GF>, String>
where
    GF: PrimeField,
{
    if inputs.len() == 0 {
        return Err("Empty inputs".to_string());
    }
    let n_remaining = inputs.len() % params.rate;
    if n_remaining != 0 {
        let message = "Input length ".to_string() + &inputs.len().to_string() + " must be a multiple of the hash rate " + &params.rate.to_string();
        return Err(message);
    }

    let mut poseidon = Poseidon::<'a, GF>::new(&params);
    for input in inputs {
        poseidon.absorb(input);
    }
    let mut result = vec![GF::ZERO; params.output_size];
    for i in 0..params.output_size {
        result[i] = poseidon.squeeze();
    }
    Ok(result)
}

#[cfg(test)]
mod test_permutation {
    use super::*;
    use crate::convert::felts_from_str;
    use crate::parameters::s128b::{GF, PARAMS};

    #[test]
    fn test_ark() {
        let mut poseidon = Poseidon::<GF>::new(&PARAMS);
        let input = vec![GF::from(7), GF::from(98), GF::from(0)];
        poseidon.state.clone_from(&input);
        poseidon.ark(0);
        let expected = [
            "10187801339791605336251748402605479409606566396373491958667041943798551150218",
            "8824452141556477327634835943439996420519135454314677708228322513850226510123",
            "5264468709835621148349527988912247104353814123939106227116596276180070073104",
        ];
        let expected = felts_from_str::<GF>(&expected);
        assert_eq!(poseidon.state, expected);
    }

    #[test]
    fn test_sbox_full() {
        let mut poseidon = Poseidon::<GF>::new(&PARAMS);
        let state = [
            "10187801339791605336251748402605479409606566396373491958667041943798551150218",
            "8824452141556477327634835943439996420519135454314677708228322513850226510123",
            "5264468709835621148349527988912247104353814123939106227116596276180070073104",
        ];
        poseidon.state.clone_from(&felts_from_str::<GF>(&state));
        poseidon.sbox_full();
        let expected = [
            "9033127700447853090229678702028773675793347128105171639302548972716183808266",
            "12584005788907507820847858681541330081079761745009746063606627523756483557914",
            "481502024243180892202073663390242313819723218601880119953632240938269076973",
        ];
        let expected = felts_from_str::<GF>(&expected);
        assert_eq!(poseidon.state, expected);
    }

    #[test]
    fn test_sbox_partial() {
        let mut poseidon = Poseidon::<GF>::new(&PARAMS);
        let state = [
            "10187801339791605336251748402605479409606566396373491958667041943798551150218",
            "8824452141556477327634835943439996420519135454314677708228322513850226510123",
            "5264468709835621148349527988912247104353814123939106227116596276180070073104",
        ];
        poseidon.state.clone_from(&felts_from_str::<GF>(&state));
        poseidon.sbox_partial();
        let expected = [
            "10187801339791605336251748402605479409606566396373491958667041943798551150218",
            "8824452141556477327634835943439996420519135454314677708228322513850226510123",
            "481502024243180892202073663390242313819723218601880119953632240938269076973",
        ];
        let expected = felts_from_str::<GF>(&expected);
        assert_eq!(poseidon.state, expected);
    }

    #[test]
    fn test_mix() {
        let mut poseidon = Poseidon::<GF>::new(&PARAMS);
        let state = [
            "9033127700447853090229678702028773675793347128105171639302548972716183808266",
            "12584005788907507820847858681541330081079761745009746063606627523756483557914",
            "481502024243180892202073663390242313819723218601880119953632240938269076973",
        ];
        poseidon.state.clone_from(&felts_from_str::<GF>(&state));
        poseidon.mix();
        let expected = [
            "13203118710027330771388479782000018409212326464085107030609226142628639238173",
            "5548559894030014093638382051049588462182080648170927883872275099718526588448",
            "13953418491398777380386807509733024828619099654861449366171745749260147070969",
        ];
        let expected = felts_from_str::<GF>(&expected);
        assert_eq!(poseidon.state, expected);
    }

    #[test]
    fn test_permutation() {
        let mut poseidon = Poseidon::<GF>::new(&PARAMS);
        let input = vec![GF::from(7), GF::from(98), GF::from(0)];
        poseidon.state.clone_from(&input);
        poseidon.permute();
        let expected = [
            "11053447091811250430558990262025664436943237361628909971717799705027922243051",
            "13019049864140962728034369799908465523605511214062523575537207652022843673387",
            "1746872083923296042231738669780413133264403148587998064386629453972513813613",
        ];
        let expected = felts_from_str::<GF>(&expected);
        assert_eq!(poseidon.state, expected);
    }
}
