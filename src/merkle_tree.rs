#[cfg(test)]
mod tests {
    use crate::tree::build_tree::{build_tree, hashed_leafs, leaf, to_hex_string};

    #[test]
    fn test() {
        let number_of_leaf = 8;
        let leafs = hashed_leafs(leaf(number_of_leaf));
        println!("Leafs:{:?}", to_hex_string(leafs.clone()));
        build_tree(leafs);
    }
}
