use bintree::Tree;
use std::fmt;

pub fn hbal_trees<T: fmt::Display + Copy>(h: usize, v: T) -> Vec<Tree<T>> {
    if h == 0 {
        vec![Tree::end()]
    } else if h == 1 {
        vec![Tree::leaf(v)]
    } else {
        let subtree1 = hbal_trees(h - 1, v);
        let subtree2 = hbal_trees(h - 2, v);
        let mut res = vec![];
        let trees1: Vec<Tree<T>> = subtree1
            .iter()
            .flat_map(|s1| {
                subtree1
                    .iter()
                    .map(move |s2| Tree::node(v, s1.clone(), s2.clone()))
            })
            .collect();
        res.extend_from_slice(&trees1);
        let trees2: Vec<Tree<T>> = subtree1
            .iter()
            .flat_map(|s1| {
                subtree2.iter().flat_map(move |s2| {
                    vec![
                        Tree::node(v, s1.clone(), s2.clone()),
                        Tree::node(v, s2.clone(), s1.clone()),
                    ]
                })
            })
            .collect();
        res.extend_from_slice(&trees2);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hbal_trees_0() {
        let trees = hbal_trees(0, '_');
        assert_eq!(trees, vec![Tree::end()]);
    }

    #[test]
    fn test_hbal_trees_1() {
        let trees = hbal_trees(1, 'x');
        assert_eq!(trees, vec![Tree::leaf('x')]);
    }

    #[test]
    fn test_hbal_trees_2() {
        let trees = hbal_trees(2, 'x');
        assert_eq!(
            trees,
            vec![
                Tree::node(
                    'x',
                    Tree::node('x', Tree::end(), Tree::end()),
                    Tree::node('x', Tree::end(), Tree::end())
                ),
                Tree::node('x', Tree::node('x', Tree::end(), Tree::end()), Tree::end()),
                Tree::node('x', Tree::end(), Tree::node('x', Tree::end(), Tree::end()))
            ]
        )
    }

    #[test]
    fn test_hbal_trees3() {
        let trees = hbal_trees(3, 'x');
        assert_eq!(trees.len(), 15);
    }
}
