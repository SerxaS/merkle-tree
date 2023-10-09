use super::build_tree::MerkleTree;
use crate::poseidon::sponge::PoseidonSponge;
use halo2::halo2curves::bn256::Fr;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Path<const H: usize> {
    pub(crate) path: [[Fr; 2]; H],
}

impl<const H: usize> Path<H> {
    pub fn find_path<const HEIGHT: usize>(tree: MerkleTree, leaf_for_proof_hash: Fr) -> Self {
        let mut path_values = HashMap::new();
        let mut proof_tree = HashMap::new();
        let mut layer_idx = 1;
        let mut path_idx = 0;
        proof_tree.insert(layer_idx, leaf_for_proof_hash);

        for i in 1..tree.node.len() as u32 {
            layer_idx += 1;
            path_idx += 1;
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
                path_values.insert(path_idx, concatenate_leaves);
            }
        }
        path_values.insert(H, [tree.root, Fr::zero()]);
        let mut binary_values = [[Fr::zero(); 2]; H];
        binary_values[H - 1] = path_values[&H];

        for i in 1..H {
            binary_values[i - 1] = path_values[&(i)];
        }
        let path = Path {
            path: binary_values,
        };
        path
    }

    pub fn verify(&self, tree: MerkleTree) -> bool {
        let mut proof_root = [Fr::zero(); H];

        for i in 0..(H - 1) {
            let mut sponge = PoseidonSponge::new();
            sponge.update(&self.path[i]);
            proof_root[i] = PoseidonSponge::squeeze(&mut sponge);
        }

        if proof_root[H - 2] == tree.root {
            true
        } else {
            false
        }
    }
}
