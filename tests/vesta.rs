use poseidon::convert::felts_from_str;
use poseidon::hash_vesta as hash;
use poseidon::parameters::vesta::GF;

#[test]
fn test_hash_simple() {
    let input = ["0", "0"];
    let input = felts_from_str::<GF>(&input);
    let expected = [
        "15503507843680028074813146032292970642340699155253454797233283122108694118843",
        "304291385369895909791985322046133629782080136045317966776007874226152902593",
    ];
    let expected = felts_from_str::<GF>(&expected);
    let output = hash(&input);
    assert_eq!(output, expected);
}

#[test]
#[should_panic]
fn test_hash_wrong_size() {
    let input = ["0", "7", "98", "0", "0"];
    let input = felts_from_str::<GF>(&input);
    hash(&input);
}
