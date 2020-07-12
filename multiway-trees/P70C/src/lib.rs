use mtree::MTree;

pub fn node_count(tree: &MTree) -> usize {
    let sum_child_nodes: usize = tree.get_children().iter().map(|t| node_count(t)).sum();
    1 + sum_child_nodes
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_node_count() {
        assert_eq!(node_count(&MTree::leaf('a')), 1);
        assert_eq!(node_count(&MTree::node('a', vec![MTree::leaf('f')])), 2);
        assert_eq!(
            node_count(&MTree::node(
                'a',
                vec![
                    MTree::node('f', vec![MTree::leaf('g')]),
                    MTree::leaf('c'),
                    MTree::node('b', vec![MTree::leaf('d'), MTree::leaf('e')])
                ]
            )),
            7
        );
    }
}
