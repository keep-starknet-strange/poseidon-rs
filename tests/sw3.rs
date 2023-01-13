use poseidon::convert::felts_from_str;
use poseidon::hash_sw3 as hash;
use poseidon::parameters::sw3::GF;

#[test]
fn test_hash_simple() {
    let input = ["0", "0", "0"];
    let input = felts_from_str::<GF>(&input);
    let expected = [
        "535071095200566880914603862188010633478042591441142518549720701573192347548",
        "3567335813488551850156302853280844225974867890860330236555401145692518003968",
        "229995103310401763929738317978722680640995513996113588430855556460153357543",
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
