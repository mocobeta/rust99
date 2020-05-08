use bintree::Tree;
use std::fmt;

pub fn leaf_list<T: Copy + fmt::Display>(tree: &Tree<T>) -> Vec<T> {
    match tree {
        Tree::Node { value, left, right } => match (left.as_ref(), right.as_ref()) {
            (Tree::Node { .. }, Tree::Node { .. }) => {
                let mut leaves = leaf_list(left);
                leaves.extend_from_slice(&leaf_list(right));
                leaves
            }
            (Tree::Node { .. }, Tree::End) => leaf_list(left),
            (Tree::End, Tree::Node { .. }) => leaf_list(right),
            (Tree::End, Tree::End) => vec![*value],
        },
        Tree::End => vec![],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leaf_list() {
        assert_eq!(leaf_list(&Tree::<char>::end()), vec![]);
        assert_eq!(leaf_list(&Tree::leaf('a')), vec!['a']);
        assert_eq!(
            leaf_list(&Tree::node(
                'a',
                Tree::leaf('b'),
                Tree::node('c', Tree::leaf('d'), Tree::leaf('e'))
            )),
            vec!['b', 'd', 'e']
        );
    }
}
