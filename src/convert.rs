use ff::PrimeField;


pub fn felts_from_str<GF> (constants: &[&'static str]) -> Vec<GF>
where
    GF: PrimeField
{
    let mut result = vec![GF::ZERO; constants.len()];
    for i in 0..constants.len() {
        result[i] = GF::from_str_vartime(constants[i]).unwrap();
    }
    result
}

pub fn scalar_from_u8s<GF>(parts: &[u8]) -> GF
where
    GF: PrimeField,
{
    let mut repr = GF::Repr::default();
    repr.as_mut().copy_from_slice(&parts[..]);
    GF::from_repr_vartime(repr).expect("u64s exceeds field modulus")
}

pub fn felts_from_u8s<GF>(parts: &[u8]) -> Vec<GF>
where
    GF: PrimeField,
{
    let n_bytes = GF::ZERO.to_repr().as_ref().len();
    if parts.len() % n_bytes != 0 {
        panic!(
            "Incorrect length for felts: {} is not a multiple of {}",
            parts.len(),
            n_bytes
        );
    }
    let size = parts.len() / n_bytes;
    let mut output = vec![GF::ZERO; size];
    for i in (0..parts.len()).step_by(n_bytes) {
        output[i / n_bytes] = scalar_from_u8s(&parts[i..(i + n_bytes)]);
    }
    output
}

pub fn u8s_from_felts<GF>(state: &[GF]) -> Vec<u8>
where
    GF: PrimeField,
{
    // let num_bytes_per_felt = (len as usize) / state.len();
    let n_bytes = GF::ZERO.to_repr().as_ref().len();
    let size = state.len() * n_bytes;
    let mut output = vec![0u8; size];
    for i in (0..size).step_by(n_bytes) {
        output[i..(i + n_bytes)].copy_from_slice(state[i / n_bytes].to_repr().as_ref());
    }
    output
}


pub fn scalar_from_u64s<GF>(parts: &[u64]) -> GF
where
    GF: PrimeField,
{
    let mut le_bytes = vec![0u8; parts.len() * 8];
    for i in (0..le_bytes.len()).step_by(8) {
        le_bytes[i..(i + 8)].copy_from_slice(&parts[i / 8].to_le_bytes());
    }
    let mut repr = <GF as PrimeField>::Repr::default();
    repr.as_mut().copy_from_slice(&le_bytes[..]);
    GF::from_repr_vartime(repr).expect("u64s exceeds field modulus")
}
