#[cfg(test)]
mod tests {
    use crate::tree::build_tree::{build_tree, hashed_leaves, leaf, to_hex_string};

    #[test]
    fn test() {
        let number_of_leaf = 8;
        let leaves = hashed_leaves(leaf(number_of_leaf));
        println!("Leaves:{:?}", to_hex_string(leaves.clone()));
        build_tree(leaves);
    }
}
