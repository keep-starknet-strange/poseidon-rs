use poseidon::convert::felts_from_str;
use poseidon::parameters::sw4::GF;
use poseidon::hash_sw4 as hash;


#[test]
fn test_hash_simple() {
    let input = ["0", "0", "0", "0"];
    let input = felts_from_str::<GF>(&input);
    let expected = [
        "2337689130971531876049206831496963607805116499042700598724344149414565980684",
        "3230969295497815870174763682436655274044379544854667759151474216427142025631",
        "3297330512217530111610698859408044542971696143761201570393504997742535648562",
        "2585480844700786541432072704002477919020588246983274666988914431019064343941",
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
