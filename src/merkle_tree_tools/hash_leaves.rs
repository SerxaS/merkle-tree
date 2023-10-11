//! # Hashes given field elements with PoseidonHash.
use crate::poseidon::sponge::PoseidonSponge;
use halo2::halo2curves::bn256::Fr;

pub struct HashLeaves {
    pub(crate) leaves: Vec<Fr>,
}

impl HashLeaves {
    pub(crate) fn new(leaves: Vec<Fr>) -> Self {
        Self { leaves }
    }

    /// Hashes given Fr values.
    pub(crate) fn hash_leaves(&self) -> Self {
        let mut hashed_leaves = HashLeaves::new(Vec::new());

        for i in &self.leaves {
            let mut leaf = Vec::new();
            leaf.push(*i);
            let mut sponge = PoseidonSponge::new();
            sponge.update(&leaf);
            let squeeze = PoseidonSponge::squeeze(&mut sponge);
            hashed_leaves.leaves.push(squeeze);
        }
        hashed_leaves
    }
}
