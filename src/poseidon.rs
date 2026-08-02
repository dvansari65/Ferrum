use light_poseidon::{Poseidon, PoseidonError, PoseidonHasher, parameters::bn254_x5};
use ark_bn254::Fr;

pub fn poseiden(a:&Fr,b:&Fr)-> Fr{
    let mut poseidon = Poseidon::<Fr>::new_circom(2).unwrap();
    poseidon.hash(&[*a,*b]).unwrap()
}