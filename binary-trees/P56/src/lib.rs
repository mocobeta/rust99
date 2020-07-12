use bintree::Tree;
use std::fmt;

pub fn is_symmetric<T: fmt::Display>(tree: &Tree<T>) -> bool {
    fn is_mirror_of<T: fmt::Display>(t1: &Tree<T>, t2: &Tree<T>) -> bool {
        match (t1, t2) {
            (Tree::End, Tree::End) => true,
            (Tree::End, Tree::Node { .. }) => false,
            (Tree::Node { .. }, Tree::End) => false,
            (
                Tree::Node {
                    value: _,
                    left: lleft,
                    right: lright,
                },
                Tree::Node {
                    value: _,
                    left: rleft,
                    right: rright,
                },
            ) => is_mirror_of(lleft, rright) && is_mirror_of(lright, rleft),
        }
    }

    match tree {
        Tree::Node {
            value: _,
            left,
            right,
        } => is_mirror_of(left, right),
        Tree::End => true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_symmetric_end() {
        assert_eq!(is_symmetric(&Tree::<char>::end()), true);
    }

    #[test]
    fn test_is_symmetric_leaf() {
        assert_eq!(is_symmetric(&Tree::leaf('a')), true);
    }

    #[test]
    fn test_is_symmetric() {
        assert_eq!(
            is_symmetric(&Tree::node('a', Tree::leaf('b'), Tree::leaf('c'))),
            true
        );

        assert_eq!(
            is_symmetric(&Tree::node('a', Tree::leaf('b'), Tree::end())),
            false
        );

        assert_eq!(
            is_symmetric(&Tree::node('a', Tree::end(), Tree::leaf('c'))),
            false
        );
    }

    #[test]
    fn test_is_symmetric_complex() {
        let tree = Tree::node(
            'a',
            Tree::node(
                'b',
                Tree::node('d', Tree::end(), Tree::leaf('e')),
                Tree::end(),
            ),
            Tree::node(
                'c',
                Tree::end(),
                Tree::node('f', Tree::leaf('g'), Tree::end()),
            ),
        );
        assert_eq!(is_symmetric(&tree), true);

        let tree = Tree::node(
            'a',
            Tree::node('b', Tree::leaf('d'), Tree::leaf('e')),
            Tree::node(
                'c',
                Tree::end(),
                Tree::node('f', Tree::leaf('g'), Tree::end()),
            ),
        );
        assert_eq!(is_symmetric(&tree), false);
    }
}
