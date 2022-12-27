extern crate ff;
use ff::*;


// Prime Fields
#[derive(PrimeField)]
#[PrimeFieldModulus = "14474011154664525231415395255581126252639794253786371766033694892385558855681"]
#[PrimeFieldGenerator = "7"]
pub struct F253(Repr);

// Insert other prime fields as needed
