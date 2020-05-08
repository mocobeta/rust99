use bintree::Tree;
use std::fmt;

pub fn internal_list<T: Copy + fmt::Display>(tree: &Tree<T>) -> Vec<T> {
    match tree {
        Tree::Node { value, left, right } => match (left.as_ref(), right.as_ref()) {
            (Tree::Node { .. }, Tree::Node { .. }) => {
                let mut leaves = vec![*value];
                leaves.extend_from_slice(&internal_list(left));
                leaves.extend_from_slice(&internal_list(right));
                leaves
            }
            (Tree::Node { .. }, Tree::End) => {
                let mut leaves = vec![*value];
                leaves.extend_from_slice(&internal_list(left));
                leaves
            }
            (Tree::End, Tree::Node { .. }) => {
                let mut leaves = vec![*value];
                leaves.extend_from_slice(&internal_list(right));
                leaves
            }
            (Tree::End, Tree::End) => vec![],
        },
        Tree::End => vec![],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_internal_list() {
        assert_eq!(internal_list(&Tree::<char>::end()), vec![]);
        assert_eq!(internal_list(&Tree::leaf('a')), vec![]);
        assert_eq!(
            internal_list(&Tree::node(
                'a',
                Tree::leaf('b'),
                Tree::node('c', Tree::leaf('d'), Tree::leaf('e'))
            )),
            vec!['a', 'c']
        );
    }
}
