use halo2::{arithmetic::Field, halo2curves::bn256::Fr};
use rand::thread_rng;
use tiny_keccak::{Hasher, Sha3};

pub(crate) fn leaf(number_of_leaf: u64) -> Vec<[u8; 32]> {
    let rng = thread_rng();
    let mut leafs = Vec::new();

    for _ in 0..number_of_leaf {
        let random_num = Fr::random(rng.clone()).to_bytes();
        leafs.push(random_num);
    }
    leafs
}

pub(crate) fn hashed_leafs(leafs: Vec<[u8; 32]>) -> Vec<[u8; 32]> {
    let mut hashed_leafs = Vec::new();

    for i in leafs {
        let mut hasher = Sha3::v256();
        hasher.update(&i);
        let mut output = [0u8; 32];
        hasher.finalize(&mut output);
        hashed_leafs.push(output);
    }
    hashed_leafs
}

pub(crate) fn build_tree(hashed_leafs: Vec<[u8; 32]>) -> Vec<[u8; 32]> {
    if hashed_leafs.len() == 0 {
        println!("{}", "Give a data please");
        return hashed_leafs;
    }

    if hashed_leafs.len() == 1 {
        return hashed_leafs;
    }

    let mut layer: Vec<[u8; 32]> = Vec::new();

    for i in (0..hashed_leafs.len()).step_by(2) {
        let left_leaf = hashed_leafs[i];
        let right_leaf = if i + 1 < hashed_leafs.len() {
            hashed_leafs[i + 1]
        } else {
            left_leaf
        };
        let concat_leafs = [left_leaf, right_leaf];
        let mut hasher = Sha3::v256();
        hasher.update(&concat_leafs.concat());
        let mut output = [0u8; 32];
        hasher.finalize(&mut output);
        layer.push(output);
    }

    let mut hexed_leaf = Vec::new();

    for i in layer.clone() {
        let mut byte_leafs = Vec::new();
        byte_leafs.push(i);

        for j in byte_leafs {
            let strs: Vec<String> = j.iter().map(|b| format!("{:x}", b)).collect();
            hexed_leaf.push(strs.join(""));
        }
    }
    build_tree(layer)
}

pub(crate) fn to_hex_string(hashed_leafs: Vec<[u8; 32]>) -> Vec<String> {
    let mut hexed_leaf = Vec::new();

    for i in hashed_leafs {
        let mut byte_leafs = Vec::new();
        byte_leafs.push(i);

        for j in byte_leafs {
            let strs: Vec<String> = j.iter().map(|b| format!("{:02x}", b)).collect();
            hexed_leaf.push(strs.join(""));
        }
    }
    hexed_leaf
}
