use bintree::Tree;
use std::fmt;
use P55::cbal_trees;
use P56::is_symmetric;

pub fn symmetric_balanced_trees<T: Copy + fmt::Display>(n: usize, v: T) -> Vec<Tree<T>> {
    cbal_trees(n, v)
        .into_iter()
        .filter(|t| is_symmetric(&t))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symmetric_balanced_trees() {
        let trees = symmetric_balanced_trees(5, 'x');
        assert!(trees.contains(&Tree::node(
            'x',
            Tree::node('x', Tree::end(), Tree::node('x', Tree::end(), Tree::end())),
            Tree::node('x', Tree::node('x', Tree::end(), Tree::end()), Tree::end())
        )));
        assert!(trees.contains(&Tree::node(
            'x',
            Tree::node('x', Tree::node('x', Tree::end(), Tree::end()), Tree::end()),
            Tree::node('x', Tree::end(), Tree::node('x', Tree::end(), Tree::end()))
        )));
    }
}
