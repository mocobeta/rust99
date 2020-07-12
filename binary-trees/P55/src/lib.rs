use bintree::Tree;
use std::fmt;

pub fn cbal_trees<T: Copy + fmt::Display>(n: usize, v: T) -> Vec<Tree<T>> {
    if n == 0 {
        vec![Tree::end()]
    } else if (n - 1) % 2 == 0 {
        let subtrees = cbal_trees((n - 1) / 2, v);
        subtrees
            .iter()
            .flat_map(|s1| {
                subtrees
                    .iter()
                    .map(move |s2| Tree::node(v, s1.clone(), s2.clone())) // left and right child has an equal number of nodes
            })
            .collect()
    } else {
        let subtrees1 = cbal_trees((n - 1) / 2, v);
        let subtrees2 = cbal_trees((n - 1) / 2 + 1, v);
        subtrees1
            .iter()
            .flat_map(|s1| {
                subtrees2.iter().flat_map(move |s2| {
                    vec![
                        Tree::node(v, s1.clone(), s2.clone()), // right child has one more node than left child
                        Tree::node(v, s2.clone(), s1.clone()), // left child has one more node than right child
                    ]
                })
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cbal_tree_1() {
        let tree = cbal_trees(1, 'x');
        assert_eq!(tree, vec![Tree::leaf('x')]);
    }

    #[test]
    fn test_cbal_tree_2() {
        let tree = cbal_trees(2, 'x');
        assert_eq!(
            tree,
            vec![
                Tree::node('x', Tree::end(), Tree::leaf('x')),
                Tree::node('x', Tree::leaf('x'), Tree::end()),
            ]
        );
    }

    #[test]
    fn test_cbal_tree_3() {
        let tree = cbal_trees(3, 'x');
        assert_eq!(
            tree,
            vec![Tree::node('x', Tree::leaf('x'), Tree::leaf('x'))]
        );
    }

    #[test]
    fn test_cbal_tree_4() {
        let tree = cbal_trees(4, 'x');
        assert_eq!(tree.len(), 4);
        assert!(tree.contains(&Tree::node(
            'x',
            Tree::node('x', Tree::leaf('x'), Tree::end()),
            Tree::node('x', Tree::end(), Tree::end())
        )));
        assert!(tree.contains(&Tree::node(
            'x',
            Tree::node('x', Tree::end(), Tree::leaf('x')),
            Tree::node('x', Tree::end(), Tree::end())
        )));
        assert!(tree.contains(&Tree::node(
            'x',
            Tree::node('x', Tree::end(), Tree::end()),
            Tree::node('x', Tree::leaf('x'), Tree::end())
        )));
        assert!(tree.contains(&Tree::node(
            'x',
            Tree::node('x', Tree::end(), Tree::end()),
            Tree::node('x', Tree::end(), Tree::leaf('x'))
        )));
    }
}
