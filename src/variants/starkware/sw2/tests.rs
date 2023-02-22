use super::*;
use crate::{fields::Field, permutation::Sponge};
use core::marker::PhantomData;

#[test]
fn test_hash_simple() {
    let input = [GF::ZERO, GF::ZERO];
    // Expected in Integer
    // 3446325744004048536138401612021367625846492093718951375866996507163446763827
    // 1590252087433376791875644726012779423683501236913937337746052470473806035332
    let expected = [
        GF {
            repr: [
                9679659289224438067,
                9593607935225213411,
                11713803070837308358,
                549031366590038016,
            ],
            phantom: PhantomData,
        },
        GF {
            repr: [
                11697560501447801220,
                16552206698782960316,
                15832289038794401339,
                253341773715160982,
            ],
            phantom: PhantomData,
        },
    ];
    // F{repr: [9807529923596045758, 16536855018464238055, 14664526698940491102, 432130875968301967], phantom: PhantomData},
    // F{repr: [4063038782967151671, 13173406895804981287, 11277348039386025686, 436343559696960954], phantom: PhantomData},
    let mut spg = Poseidon {
        state: [GF::ZERO; 3],
        constants: &CONSTANTS,
    };
    spg.absorb(&input);
    let output = spg.squeeze();
    assert_eq!(output, expected);
}
