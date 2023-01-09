// use libc;
// use libc::size_t;
use ff::PrimeField;

pub fn scalar_from_u64s<Fr>(parts: &[u64]) -> Fr
where
    Fr: PrimeField,
{
    let mut le_bytes = vec![0u8; parts.len() * 8];
    for i in (0..le_bytes.len()).step_by(8) {
        le_bytes[i..(i + 8)].copy_from_slice(&parts[i / 8].to_le_bytes());
    }
    let mut repr = <Fr as PrimeField>::Repr::default();
    repr.as_mut().copy_from_slice(&le_bytes[..]);
    Fr::from_repr_vartime(repr).expect("u64s exceeds field modulus")
}

pub fn scalar_from_u8s<Fr>(parts: &[u8]) -> Fr
where
    Fr: PrimeField,
{
    let mut repr = Fr::Repr::default();
    repr.as_mut().copy_from_slice(&parts[..]);
    Fr::from_repr_vartime(repr).expect("u64s exceeds field modulus")
}

pub fn felts_from_u8s<Fr>(parts: &[u8]) -> Vec<Fr>
where
    Fr: PrimeField,
{
    let n_bytes = Fr::ZERO.to_repr().as_ref().len();
    if parts.len() % n_bytes != 0 {
        panic!(
            "Incorrect length for felts: {} is not a multiple of {}",
            parts.len(),
            n_bytes
        );
    }
    let size = parts.len() / n_bytes;
    let mut output = vec![Fr::ZERO; size];
    for i in (0..parts.len()).step_by(n_bytes) {
        output[i / n_bytes] = scalar_from_u8s(&parts[i..(i + n_bytes)]);
    }
    output
}

pub fn u8s_from_felts<Fr>(state: &[Fr]) -> Vec<u8>
where
    Fr: PrimeField,
{
    // let num_bytes_per_felt = (len as usize) / state.len();
    let n_bytes = Fr::ZERO.to_repr().as_ref().len();
    let size = state.len() * n_bytes;
    let mut output = vec![0u8; size];
    for i in (0..size).step_by(n_bytes) {
        output[i..(i + n_bytes)].copy_from_slice(state[i / n_bytes].to_repr().as_ref());
    }
    output
}

// pub extern fn (state, buffer *mut u8, len: size_t) {
//     unsafe {
//         if (len as usize) > size {
//             len = size;
//         }
//         std::ptr::copy_nonoverlapping(&output, buffer, len as usize);
//     }
// }

// pub fn u64s_from_scalar<Fr>(elt: Fr) -> Vec<u64>
// where
//     Fr: PrimeField
// {
//     let repr = elt.to_repr();
//     let mut le_bytes = vec![0u8; repr.len()];
//     le_bytes.copy_from_slice(&repr);
//     let output = vec![0u64; 4];
//     output
// }
