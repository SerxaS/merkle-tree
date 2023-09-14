#[cfg(test)]
mod tests {
    use crate::tree::build_tree::{build_tree, hashed_leafs, leaf, to_hex_string};

    #[test]
    fn test() {
        let number_of_leaf = 8;
        let merkle_root = build_tree(hashed_leafs(leaf(number_of_leaf)));
        println!("Merkle Root:{:?}", to_hex_string(merkle_root)[0]);
    }
}
