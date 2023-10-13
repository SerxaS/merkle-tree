//! # Merkle Tree
#[cfg(test)]
mod tests {
    use crate::merkle_tree_tools::{
        build_tree::MerkleTree,
        hash_leaves::HashLeaves,
        proof::{self, Proof},
    };
    use halo2::{arithmetic::Field, halo2curves::bn256::Fr};
    use rand::thread_rng;

    #[test]
    fn full_tree() {
        // Creates first layer of tree with given number.
        let number_of_leaf = 5;
        // Builds a tree with given number, i.e. the power of 2 that is closest to the total
        // number of leaves is height of tree(H) and 1 more is level of tree(L).
        const H: usize = 3;
        const L: usize = 4;
        assert!(H < L, "Level must be 1 more than height!");

        let rng = thread_rng();
        let leaves = &HashLeaves::new(
            (0..number_of_leaf)
                .map(|_| Fr::random(rng.clone()))
                .collect(),
        );
        let mut hashed_leaves = HashLeaves::hash_leaves(leaves);
        // Random leaf for proof. Inserts a value to the given index.
        let leaf_for_proof = Fr::random(rng);
        let leaf_for_proof_hash = HashLeaves::new(vec![leaf_for_proof]).leaves[0];
        let leaf_for_proof_idx = 2;
        hashed_leaves
            .leaves
            .insert(leaf_for_proof_idx, leaf_for_proof_hash);

        // If length of first layer is not equal to exponent of 2, adds zero until layer length
        // comes exponent of 2.
        while (hashed_leaves.leaves.len() & (hashed_leaves.leaves.len() - 1)) != 0 {
            hashed_leaves
                .leaves
                .insert(hashed_leaves.leaves.len(), Fr::zero());
        }
        let tree = MerkleTree::build_tree(hashed_leaves);
        let proof: proof::Proof<H, L> = Proof::find_path(tree.clone(), leaf_for_proof_hash);
        // Verifier hashes the given path's values and finds the root then compare it original root.
        let verify = Proof::verify(&proof, tree);
        assert!(verify);
    }

    #[test]
    fn half_tree() {
        // Creates first layer of tree with given number.
        let number_of_leaf = 2;
        // Builds a tree with given number, i.e. the power of 2 that is closest to the total
        // number of leaves is height of tree(H) and 1 more is level of tree(L).
        const H: usize = 5;
        const L: usize = 6;
        assert!(H < L, "Level must be 1 more than height!");

        let rng = thread_rng();
        let leaves = &HashLeaves::new(
            (0..number_of_leaf)
                .map(|_| Fr::random(rng.clone()))
                .collect(),
        );
        let mut hashed_leaves = HashLeaves::hash_leaves(leaves);
        // Random leaf for proof. Inserts a value to the given index.
        let leaf_for_proof = Fr::random(rng);
        let leaf_for_proof_hash = HashLeaves::new(vec![leaf_for_proof]).leaves[0];
        let leaf_for_proof_idx = 2;
        hashed_leaves
            .leaves
            .insert(leaf_for_proof_idx, leaf_for_proof_hash);

        // If given values are not enough to fill tree, adds zero until layer reaches enough lenght.
        loop {
            if hashed_leaves.leaves.len() < 2_usize.pow(H.try_into().unwrap()) {
                hashed_leaves
                    .leaves
                    .insert(hashed_leaves.leaves.len(), Fr::zero());
            } else {
                break;
            }
        }
        let tree = MerkleTree::build_tree(hashed_leaves);
        let proof: proof::Proof<H, L> = Proof::find_path(tree.clone(), leaf_for_proof_hash);
        // Verifier hashes the given path's values and finds the root then compare it original root.
        let verify = Proof::verify(&proof, tree);
        assert!(verify);
    }

    #[test]
    fn empty_tree() {
        // Empty tree.
        let number_of_leaf = 0;
        // Builds a tree with given number, i.e. the power of 2 that is closest to the total
        // number of leaves is height of tree(H).
        const H: usize = 3;

        let rng = thread_rng();
        let mut leaves = HashLeaves::new(
            (0..number_of_leaf)
                .map(|_| Fr::random(rng.clone()))
                .collect(),
        );

        // If tree is empty fill layer with zero.
        loop {
            if leaves.leaves.len() != 2_usize.pow(H.try_into().unwrap()) {
                leaves.leaves.insert(leaves.leaves.len(), Fr::zero());
            } else {
                break;
            }
        }

        let tree = MerkleTree::build_tree(leaves);
    }
}
