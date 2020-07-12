use mtree::MTree;

pub fn postorder(tree: &MTree) -> Vec<char> {
    fn postorder_inner(tree: &MTree, acc: &mut Vec<char>) {
        tree.get_children()
            .iter()
            .for_each(|t| postorder_inner(t, acc));
        acc.push(tree.get_value());
    }

    let mut res = vec![];
    postorder_inner(tree, &mut res);
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    use P70::str_to_tree;

    #[test]
    fn test_postorder() {
        assert_eq!(postorder(&str_to_tree("a^")), vec!['a']);
        assert_eq!(postorder(&str_to_tree("ab^^")), vec!['b', 'a']);
        assert_eq!(
            postorder(&str_to_tree("ab^c^d^^")),
            vec!['b', 'c', 'd', 'a']
        );
        assert_eq!(postorder(&str_to_tree("abc^^^")), vec!['c', 'b', 'a']);
        assert_eq!(
            postorder(&str_to_tree("afg^^c^bd^e^^^")),
            vec!['g', 'f', 'c', 'd', 'e', 'b', 'a']
        )
    }
}
