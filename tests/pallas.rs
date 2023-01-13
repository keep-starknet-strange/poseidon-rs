use poseidon::convert::felts_from_str;
use poseidon::hash_pallas as hash;
use poseidon::parameters::pallas::GF;

#[test]
fn test_hash_simple() {
    let input = ["0", "0"];
    let input = felts_from_str::<GF>(&input);
    let expected = [
        "15203699030908700109218914175527927806302520478319440272484330643709021075093",
        "25101873011696671024407923684611714012645440013636881009339041864418865390740",
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
