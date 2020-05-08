use bintree::Tree;
use std::fmt;

pub fn leaf_count<T: fmt::Display>(tree: &Tree<T>) -> usize {
    match tree {
        Tree::Node {
            value: _,
            left,
            right,
        } => match (left.as_ref(), right.as_ref()) {
            (Tree::Node { .. }, Tree::Node { .. }) => leaf_count(left) + leaf_count(right),
            (Tree::Node { .. }, Tree::End) => leaf_count(left),
            (Tree::End, Tree::Node { .. }) => leaf_count(right),
            _ => 1,
        },
        Tree::End => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leaf_count() {
        assert_eq!(leaf_count(&Tree::<char>::end()), 0);
        assert_eq!(leaf_count(&Tree::leaf('a')), 1);
        assert_eq!(
            leaf_count(&Tree::node('a', Tree::leaf('b'), Tree::end())),
            1
        );
        assert_eq!(
            leaf_count(&Tree::node(
                'a',
                Tree::node('b', Tree::leaf('d'), Tree::leaf('e')),
                Tree::node(
                    'c',
                    Tree::end(),
                    Tree::node('f', Tree::leaf('g'), Tree::end()),
                )
            )),
            3
        );
    }
}
