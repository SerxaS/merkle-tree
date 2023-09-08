use halo2::{arithmetic::Field, halo2curves::bn256::Fr};
use rand::thread_rng;
use std::collections::HashMap;
use tiny_keccak::{Hasher, Sha3};

pub(crate) fn new() -> Vec<[u8; 32]> {
    let number_of_leaf = 4;
    let rng = thread_rng();
    let mut leafs = Vec::new();

    for _ in 0..number_of_leaf {
        let random_num = Fr::random(rng.clone()).to_bytes();
        leafs.push(random_num);
    }

    let mut hashed_leafs = Vec::new();
    println!("{:?}", leafs);

    for i in leafs {
        let mut hasher = Sha3::v256();
        hasher.update(&i);
        let mut output = [0u8; 32];
        hasher.finalize(&mut output);
        hashed_leafs.push(output);
    }
    hashed_leafs
}

pub(crate) fn to_hex_string(hashed_leafs: Vec<[u8; 32]>) -> Vec<String> {
    let mut hexed_leaf = Vec::new();

    for i in hashed_leafs {
        let mut byte_leafs = Vec::new();
        byte_leafs.push(i);

        for j in byte_leafs {
            let strs: Vec<String> = j.iter().map(|b| format!("{:02X}", b)).collect();
            hexed_leaf.push(strs.join(""));
        }
    }
    hexed_leaf
}

// pub(crate) fn to_hex_string(bytes: Vec<u8>) -> String {
//     let strs: Vec<String> = bytes.iter().map(|b| format!("{:02X}", b)).collect();
//     strs.join("")
// }
