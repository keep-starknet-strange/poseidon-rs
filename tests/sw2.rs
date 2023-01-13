use poseidon::convert::felts_from_str;
use poseidon::hash_sw2 as hash;
use poseidon::parameters::sw2::GF;

#[test]
fn test_hash_simple() {
    let input = ["0", "0"];
    let input = felts_from_str::<GF>(&input);
    let expected = [
        "3446325744004048536138401612021367625846492093718951375866996507163446763827",
        "1590252087433376791875644726012779423683501236913937337746052470473806035332",
        // "867921192302518434283879514999422690776342565400001269945778456016268852423",
    ];
    let expected = felts_from_str::<GF>(&expected);
    let output = hash(&input);
    assert_eq!(output, expected);
}

#[test]
fn test_hash_double() {
    let input = ["0", "0", "0", "0"];
    let input = felts_from_str::<GF>(&input);
    let expected = [
        "1078681718110415080500590645916982986266752792435201775885080862256763224244",
        "2182280876755175089975587639040776712406800076625579698240693584568128913421",
        // "3612356683795693068535081786588567152411262810015784355344249103690137756106",
    ];
    let expected = felts_from_str::<GF>(&expected);
    let output = hash(&input);
    assert_eq!(output, expected);
}

#[test]
#[should_panic]
fn test_hash_wrong_size() {
    let input = ["0", "7", "98"];
    let input = felts_from_str::<GF>(&input);
    hash(&input);
}
