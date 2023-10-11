use super::hash_leaves::HashLeaves;
use crate::poseidon::sponge::PoseidonSponge;
use halo2::halo2curves::bn256::Fr;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct MerkleTree {
    pub(crate) node: HashMap<usize, Vec<Fr>>,
    pub(crate) root: Fr,
}

impl MerkleTree {
    /// Builds tree with given Fr elements and finds the root of tree.
    pub(crate) fn build_tree(hashed_leaves: HashLeaves) -> Self {
        let mut node: HashMap<usize, Vec<Fr>> = HashMap::new();
        let mut layer_idx = 1;
        node.insert(layer_idx, hashed_leaves.leaves.clone());

        for i in 1..(hashed_leaves.leaves.len().checked_ilog2().unwrap() + 1) as usize {
            layer_idx += 1;
            let mut layer = Vec::new();

            for j in (0..node[&i].len()).step_by(2) {
                let left_leaf = node[&i][j];
                let right_leaf = node[&i][j + 1];
                let concat_leaves = [left_leaf, right_leaf];
                let mut sponge = PoseidonSponge::new();
                sponge.update(&concat_leaves);
                let squeeze = PoseidonSponge::squeeze(&mut sponge);
                layer.push(squeeze);
            }
            node.insert(layer_idx, layer.clone());
        }
        let root = node[&(node.len())][0];
        let tree = MerkleTree { node, root };
        tree
    }
}
