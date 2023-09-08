#[cfg(test)]
mod tests {
    use crate::tree::node::{new, to_hex_string};

    #[test]
    fn test() {
        let a = new();
        println!("{:?}", to_hex_string(a));
    }
}
