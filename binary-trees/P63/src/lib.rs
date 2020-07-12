use bintree::Tree;
use std::fmt;

pub fn complete_binary_tree<T: Copy + fmt::Display>(n: usize, v: T) -> Tree<T> {
    fn complete_binary_tree_rec<T: Copy + fmt::Display>(n: usize, addr: usize, v: T) -> Tree<T> {
        if n == 0 {
            Tree::end()
        } else {
            let left_child = if n >= addr * 2 {
                complete_binary_tree_rec(n, addr * 2, v)
            } else {
                Tree::end()
            };
            let right_child = if n >= addr * 2 + 1 {
                complete_binary_tree_rec(n, addr * 2 + 1, v)
            } else {
                Tree::end()
            };
            Tree::node(v, left_child, right_child)
        }
    }

    complete_binary_tree_rec(n, 1, v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complete_binary_tree() {
        assert_eq!(complete_binary_tree(0, 'x'), Tree::end());
        assert_eq!(complete_binary_tree(1, 'x'), Tree::leaf('x'));
        assert_eq!(
            complete_binary_tree(2, 'x'),
            Tree::node('x', Tree::leaf('x'), Tree::end())
        );
        assert_eq!(
            complete_binary_tree(3, 'x'),
            Tree::node('x', Tree::leaf('x'), Tree::leaf('x'))
        );
        assert_eq!(
            complete_binary_tree(4, 'x'),
            Tree::node(
                'x',
                Tree::node('x', Tree::leaf('x'), Tree::end()),
                Tree::leaf('x')
            )
        );
        assert_eq!(
            complete_binary_tree(5, 'x'),
            Tree::node(
                'x',
                Tree::node('x', Tree::leaf('x'), Tree::leaf('x')),
                Tree::leaf('x')
            )
        );
        assert_eq!(
            complete_binary_tree(6, 'x'),
            Tree::node(
                'x',
                Tree::node('x', Tree::leaf('x'), Tree::leaf('x')),
                Tree::node('x', Tree::leaf('x'), Tree::end())
            )
        );
        assert_eq!(
            complete_binary_tree(7, 'x'),
            Tree::node(
                'x',
                Tree::node('x', Tree::leaf('x'), Tree::leaf('x')),
                Tree::node('x', Tree::leaf('x'), Tree::leaf('x'))
            )
        );
    }
}
