use crate::poseidon::sponge::PoseidonSponge;
use halo2::halo2curves::bn256::Fr;
use std::collections::HashMap;


pub(crate) fn hash_leaves(leaves: Vec<Fr>) -> Vec<Fr> {
    let mut hashed_leaves = Vec::new();

    for i in leaves {
        let mut leaf = Vec::new();
        leaf.push(i);
        let mut sponge = PoseidonSponge::new();
        sponge.update(&leaf);
        let squeeze = PoseidonSponge::squeeze(&mut sponge);
        hashed_leaves.push(squeeze);
    }
    hashed_leaves
}

pub(crate) fn build_tree(hashed_leaves: Vec<Fr>) -> HashMap<u32, Vec<Fr>> {
    let mut tree = HashMap::new();
    let mut layer_idx = 1;
    tree.insert(layer_idx, hashed_leaves.clone());

    for i in 1..(hashed_leaves.len().checked_ilog2().unwrap() + 1) {
        layer_idx += 1;
        let mut layer = Vec::new();

        for j in (0..tree[&i].len()).step_by(2) {
            let left_leaf = tree[&i][j];
            let right_leaf = tree[&i][j + 1];
            let concat_leaves = [left_leaf, right_leaf];
            let mut sponge = PoseidonSponge::new();
            sponge.update(&concat_leaves);
            let squeeze = PoseidonSponge::squeeze(&mut sponge);
            layer.push(squeeze);
        }
        tree.insert(layer_idx, layer.clone());
    }
    tree
}

pub fn proof(tree: HashMap<u32, Vec<Fr>>, leaf_for_proof_hash: Fr) -> bool {
    let mut proof_tree = HashMap::new();
    let mut layer_idx = 1;
    proof_tree.insert(layer_idx, leaf_for_proof_hash);

    for i in 1..tree.len() as u32 {
        layer_idx += 1;
        let mut layer = Vec::new();
        let idx = tree[&i].iter().position(|x| x == &proof_tree[&i]).unwrap();

        for _ in 0..1 {
            let mut concat_leaves = [Fr::zero(); 2];

            if idx % 2 == 0 {
                concat_leaves = [tree[&i][idx], tree[&i][idx + 1]];
            } else {
                concat_leaves = [tree[&i][idx - 1], tree[&i][idx]];
            }

            let mut sponge = PoseidonSponge::new();
            sponge.update(&concat_leaves);
            let squeeze = PoseidonSponge::squeeze(&mut sponge);
            layer.push(squeeze);
            proof_tree.insert(layer_idx, layer[0]);
        }
    }

    if tree[&(tree.len() as u32)][0] == proof_tree[&(proof_tree.len() as u32)] {
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use halo2::{arithmetic::Field, halo2curves::bn256::Fr};
    use rand::thread_rng;

    #[test]
    fn test() {
        let number_of_leaf = 6;
        let rng = thread_rng();
        let leaves: Vec<Fr> = (0..number_of_leaf)
            .map(|_| Fr::random(rng.clone()))
            .collect();

        let mut hashed_leaves = hash_leaves(leaves); 

        let leaf_for_proof = Fr::random(rng);
        let leaf_for_proof_hash = hash_leaves(vec![leaf_for_proof])[0];
        hashed_leaves.insert(3, leaf_for_proof_hash);

        while (hashed_leaves.len() & (hashed_leaves.len() - 1)) != 0 {
            hashed_leaves.insert(hashed_leaves.len(), Fr::zero());
        }

        let tree = build_tree(hashed_leaves);
        println!("{:?}", tree);

        let proof = proof(tree, leaf_for_proof_hash);
        println!("{:?}", proof);
    }
}
