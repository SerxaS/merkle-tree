use crate::poseidon::sponge::PoseidonSponge;
use halo2curves::bn256::Fr;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct MerkleTree {
    pub(crate) node: HashMap<usize, Vec<Fr>>,
    pub(crate) root: Fr,
}

impl MerkleTree {
    /// Hashes given Fr values.
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

    /// Builds tree with given Fr elements and finds the root of tree.
    pub(crate) fn build_tree<const H: usize>(hashed_leaves: Vec<Fr>) -> Self {
        let mut node = HashMap::new();
        // 0th level is the leaf level and the max level is the root level.
        let mut layer_idx = 0;
        node.insert(layer_idx, hashed_leaves.clone());
        for i in 0..H {
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
        let root = node[&(node.len() - 1)][0];
        println!("{:#?}", node.len());
        MerkleTree { node, root }
    }
}
