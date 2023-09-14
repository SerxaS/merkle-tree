use halo2::{arithmetic::Field, halo2curves::bn256::Fr};
use rand::thread_rng;
use tiny_keccak::{Hasher, Sha3};

pub(crate) fn leaf(number_of_leaf: u64) -> Vec<[u8; 32]> {
    let rng = thread_rng();
    let mut leaves = Vec::new();

    for _ in 0..number_of_leaf {
        let random_num = Fr::random(rng.clone()).to_bytes();
        leaves.push(random_num);
    }
    leaves
}

pub(crate) fn hashed_leaves(leaves: Vec<[u8; 32]>) -> Vec<[u8; 32]> {
    let mut hashed_leaves = Vec::new();

    for i in leaves {
        let mut hasher = Sha3::v256();
        hasher.update(&i);
        let mut output = [0u8; 32];
        hasher.finalize(&mut output);
        hashed_leaves.push(output);
    }
    hashed_leaves
}

pub(crate) fn build_tree(hashed_leaves: Vec<[u8; 32]>) -> Vec<[u8; 32]> {
    if hashed_leaves.len() == 0 {
        println!("{}", "Give a data please");
        return hashed_leaves;
    }

    if hashed_leaves.len() == 1 {
        return hashed_leaves;
    }

    let mut layer: Vec<[u8; 32]> = Vec::new();

    for i in (0..hashed_leaves.len()).step_by(2) {
        let left_leaf = hashed_leaves[i];
        let right_leaf = if i + 1 < hashed_leaves.len() {
            hashed_leaves[i + 1]
        } else {
            left_leaf
        };
        let concat_leaves = [left_leaf, right_leaf];
        let mut hasher = Sha3::v256();
        hasher.update(&concat_leaves.concat());
        let mut output = [0u8; 32];
        hasher.finalize(&mut output);
        layer.push(output);
    }

    let mut hexed_leaf = Vec::new();

    for i in layer.clone() {
        let mut byte_leaves = Vec::new();
        byte_leaves.push(i);

        for j in byte_leaves {
            let strs: Vec<String> = j.iter().map(|b| format!("{:x}", b)).collect();
            hexed_leaf.push(strs.join(""));
        }
    }
    println!("Layers of tree:{:?}", hexed_leaf);
    build_tree(layer)
}

pub(crate) fn to_hex_string(hashed_leaves: Vec<[u8; 32]>) -> Vec<String> {
    let mut hexed_leaf = Vec::new();

    for i in hashed_leaves {
        let mut byte_leaves = Vec::new();
        byte_leaves.push(i);

        for j in byte_leaves {
            let strs: Vec<String> = j.iter().map(|b| format!("{:02x}", b)).collect();
            hexed_leaf.push(strs.join(""));
        }
    }
    hexed_leaf
}
