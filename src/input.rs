use std::{fs::File, io::Read};

use ark_bn254::Fr;
use ark_serialize::CanonicalDeserialize;

pub fn prepare_input() -> Vec<Vec<Fr>> {
    let mut file = File::open("circuit/input.bin").expect("Failed to open input file");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .expect("Failed to read input file");

    CanonicalDeserialize::deserialize_uncompressed(&mut buffer.as_slice())
        .expect("Failed to deserialize input")
}
