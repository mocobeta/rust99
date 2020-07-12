use mtree::MTree;

pub fn internal_path_length(tree: &MTree) -> usize {
    fn internal_path_length_inner(tree: &MTree, depth: usize) -> usize {
        let lengths: usize = tree
            .get_children()
            .iter()
            .map(|t| internal_path_length_inner(t, depth + 1))
            .sum();
        depth + lengths
    }

    internal_path_length_inner(tree, 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use P70::str_to_tree;
    #[test]
    fn test_internal_path_length() {
        assert_eq!(internal_path_length(&str_to_tree("a^")), 0);
        assert_eq!(internal_path_length(&str_to_tree("ab^^")), 1);
        assert_eq!(internal_path_length(&str_to_tree("ab^c^d^^")), 3);
        assert_eq!(internal_path_length(&str_to_tree("abc^^^")), 3);
        assert_eq!(internal_path_length(&str_to_tree("afg^^c^bd^e^^^")), 9);
    }
}
