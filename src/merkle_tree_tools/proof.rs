use super::build_tree::MerkleTree;
use crate::poseidon::sponge::PoseidonSponge;
use halo2::halo2curves::bn256::Fr;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Path {
    pub(crate) path: [Fr; 2],
    pub(crate) leaf_for_proof: Fr,
}

impl Path {
    pub fn find_path(tree: MerkleTree, leaf_for_proof_hash: Fr) -> Self {
        let mut proof_tree = HashMap::new();
        let mut layer_idx = 1;
        proof_tree.insert(layer_idx, leaf_for_proof_hash);
        let mut concat_leaves = [Fr::zero(); 2];

        for i in 1..tree.node.len() as u32 {
            layer_idx += 1;
            let mut layer = Vec::new();
            let idx = tree.node[&i]
                .iter()
                .position(|x| x == &proof_tree[&i])
                .unwrap();

            for _ in 0..1 {
                if idx % 2 == 0 {
                    concat_leaves = [tree.node[&i][idx], tree.node[&i][idx + 1]];
                } else {
                    concat_leaves = [tree.node[&i][idx - 1], tree.node[&i][idx]];
                }

                let mut sponge = PoseidonSponge::new();
                sponge.update(&concat_leaves);
                let squeeze = PoseidonSponge::squeeze(&mut sponge);
                layer.push(squeeze);
                proof_tree.insert(layer_idx, layer[0]);
            }
        }
        let path = Path {
            path: concat_leaves,
            leaf_for_proof: leaf_for_proof_hash,
        };
        path
    }

    pub fn verify(&self, tree: MerkleTree) -> bool {
        let mut sponge = PoseidonSponge::new();
        sponge.update(&self.path);
        let proof_root = PoseidonSponge::squeeze(&mut sponge);

        if proof_root == tree.root {
            true
        } else {
            false
        }
    }
}
