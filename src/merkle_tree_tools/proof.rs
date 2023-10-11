use super::build_tree::MerkleTree;
use crate::poseidon::sponge::PoseidonSponge;
use halo2::halo2curves::bn256::Fr;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Proof<const H: usize, const L: usize> {
    pub(crate) path: [[Fr; 2]; L],
}

impl<const H: usize, const L: usize> Proof<H, L> {
    /// Finds the path of value for proof.
    pub fn find_path(tree: MerkleTree, leaf_for_proof_hash: Fr) -> Self {
        let mut path = [[Fr::zero(); 2]; L];
        path[H] = [tree.root, Fr::zero()];
        let mut proof_tree = HashMap::new();
        let mut layer_idx = 1;
        proof_tree.insert(layer_idx, leaf_for_proof_hash);

        for i in 1..tree.node.len() {
            layer_idx += 1;
            let mut layer = Vec::new();
            let idx = tree.node[&i]
                .iter()
                .position(|x| x == &proof_tree[&i])
                .unwrap();

            for _ in 0..1 {
                let mut concatenate_leaves = [Fr::zero(); 2];

                if idx % 2 == 0 {
                    concatenate_leaves = [tree.node[&i][idx], tree.node[&i][idx + 1]];
                } else {
                    concatenate_leaves = [tree.node[&i][idx - 1], tree.node[&i][idx]];
                }
                let mut sponge = PoseidonSponge::new();
                sponge.update(&concatenate_leaves);
                let squeeze = PoseidonSponge::squeeze(&mut sponge);
                layer.push(squeeze);
                proof_tree.insert(layer_idx, layer[0]);
                path[i - 1] = concatenate_leaves;
            }
        }
        Proof { path }
    }

    /// Finds the path's root and compare it with original root.
    pub fn verify(&self, tree: MerkleTree) -> bool {
        let mut proof_root = [Fr::zero(); H];

        for i in 0..H {
            let mut sponge = PoseidonSponge::new();
            sponge.update(&self.path[i]);
            proof_root[i] = PoseidonSponge::squeeze(&mut sponge);
        }

        if *proof_root.last().unwrap() == tree.root {
            true
        } else {
            false
        }
    }
}
