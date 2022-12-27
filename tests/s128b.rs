use poseidon;
use poseidon::parameters::s128b::F253;
use poseidon::hash_s128b as hash;
use ff::{PrimeField, Field};

mod common;


// static to_felts: fn(&[&str]) -> Vec<F253> = load_felts::<F253>;

fn to_felts(input: &[&str]) -> Vec<F253> {
    common::load_felts(input)
}


#[test]
fn test_ff() {
    let b0: F253 = F253::from(0);
    let b1: F253 = F253::from(7);
    let b2: F253 = F253::from(343);

    assert_eq!(F253::ZERO, b0);
    assert_eq!(F253::MULTIPLICATIVE_GENERATOR, b1); 
    assert_eq!(b1.pow([3]), b2);
}

#[test]
fn test_hash_simple() {
    let input = [
        "7",
        "98",
    ];
    let input = to_felts(&input);
    let expected = [
        "11053447091811250430558990262025664436943237361628909971717799705027922243051",
    ];
    let expected = to_felts(&expected);
    let output = hash(&input);
    assert_eq!(output, expected);
}

#[test]
fn test_hash_double() {
    let input = [
        "7",
        "98",
        "9033127700447853090229678702028773675793347128105171639302548972716183808266", 
        "5548559894030014093638382051049588462182080648170927883872275099718526588448",
    ];
    let input = to_felts(&input);
    let expected = [
        "14226598713275173058539927170621755455377315018571595090070022785066210289324",
    ];
    let expected = to_felts(&expected);
    let output = hash(&input);
    assert_eq!(output, expected);
}

#[test]
#[should_panic]
fn test_hash_wrong_size() {
    let input = [
        "0",
        "7",
        "98",
    ];
    let input = to_felts(&input);
    hash(&input);
}
