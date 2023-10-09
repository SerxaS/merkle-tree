#[cfg(test)]
mod tests {
    use crate::merkle_tree_tools::{
        build_tree::MerkleTree,
        hash_leaves::HashLeaves,
        proof::{self, Path},
    };
    use halo2::{arithmetic::Field, halo2curves::bn256::Fr};
    use rand::thread_rng;

    #[test]
    fn test() {
        let number_of_leaf = 6;
        const H: usize = 4;
        let rng = thread_rng();
        let leaves = &HashLeaves::new(
            (0..number_of_leaf)
                .map(|_| Fr::random(rng.clone()))
                .collect(),
        );
        let mut hashed_leaves = HashLeaves::hash_leaves(leaves);
        let leaf_for_proof = Fr::random(rng);
        let leaf_for_proof_hash = HashLeaves::new(vec![leaf_for_proof]).leaves[0];
        hashed_leaves.leaves.insert(1, leaf_for_proof_hash);

        while (hashed_leaves.leaves.len() & (hashed_leaves.leaves.len() - 1)) != 0 {
            hashed_leaves
                .leaves
                .insert(hashed_leaves.leaves.len(), Fr::zero());
        }
        let tree = MerkleTree::build_tree(hashed_leaves);
        let proof: proof::Path<H> = Path::find_path::<H>(tree.clone(), leaf_for_proof_hash);
        let verify = Path::verify(&proof, tree);
        println!("{:?}", verify);
    }
}
