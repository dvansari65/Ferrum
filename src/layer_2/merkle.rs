use ark_bn254::Fr;

use crate::layer_1::posiedon::poseiden;


pub fn tree_compressor(values: Vec<Fr>) -> Fr {
    let mut current = values;

    while current.len() != 1 {
        let mut next_row = vec![];

        for pair in current.chunks(2) {
            let hash_value = poseiden(&pair[0], &pair[1]);
            next_row.push(hash_value);
        }

        current = next_row;
    }

    current[0]
}