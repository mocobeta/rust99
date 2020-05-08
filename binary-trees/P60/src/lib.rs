use bintree::Tree;
use std::fmt;
use P59::hbal_trees;

pub fn min_hbal_nodes(h: usize) -> usize {
    if h == 0 {
        0
    } else if h == 1 {
        1
    } else {
        min_hbal_nodes(h - 1) + min_hbal_nodes(h - 2) + 1
    }
}

pub fn max_hbal_height(n: usize) -> usize {
    let mut height = 0;
    loop {
        if min_hbal_nodes(height + 1) > n {
            break;
        } else {
            height += 1;
        }
    }
    height
}

pub fn hbal_trees_with_nodes<T: Copy + fmt::Display>(n: usize, v: T) -> Vec<Tree<T>> {
    let max_height = max_hbal_height(n);
    let mut res = vec![];
    for h in 0..max_height + 1 {
        if 2u32.pow(h as u32) - 1 < n as u32 {
            continue;
        }
        hbal_trees(h, v)
            .into_iter()
            .filter(|t| t.count_nodes() == n)
            .for_each(|t| res.push(t));
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_min_hbal_nodes() {
        assert_eq!(min_hbal_nodes(0), 0);
        assert_eq!(min_hbal_nodes(1), 1);
        assert_eq!(min_hbal_nodes(2), 2);
        assert_eq!(min_hbal_nodes(3), 4);
        assert_eq!(min_hbal_nodes(4), 7);
    }

    #[test]
    fn test_max_hbal_height() {
        assert_eq!(max_hbal_height(0), 0);
        assert_eq!(max_hbal_height(1), 1);
        assert_eq!(max_hbal_height(2), 2);
        assert_eq!(max_hbal_height(3), 2);
        assert_eq!(max_hbal_height(4), 3);
    }

    #[test]
    fn test_hbal_trees_with_nodes() {
        let trees = hbal_trees_with_nodes(1, 'x');
        assert_eq!(trees, vec![Tree::leaf('x')]);

        let trees = hbal_trees_with_nodes(2, 'x');
        assert_eq!(
            trees,
            vec![
                Tree::node('x', Tree::leaf('x'), Tree::end()),
                Tree::node('x', Tree::end(), Tree::leaf('x'))
            ]
        );

        let trees = hbal_trees_with_nodes(3, 'x');
        assert_eq!(
            trees,
            vec![Tree::node('x', Tree::leaf('x'), Tree::leaf('x'))]
        );

        let trees = hbal_trees_with_nodes(4, 'x');
        assert_eq!(trees.len(), 4);
        assert!(trees.contains(&Tree::node(
            'x',
            Tree::node('x', Tree::leaf('x'), Tree::end()),
            Tree::node('x', Tree::end(), Tree::end())
        )));
        assert!(trees.contains(&Tree::node(
            'x',
            Tree::node('x', Tree::end(), Tree::leaf('x')),
            Tree::node('x', Tree::end(), Tree::end())
        )));
        assert!(trees.contains(&Tree::node(
            'x',
            Tree::node('x', Tree::end(), Tree::end()),
            Tree::node('x', Tree::leaf('x'), Tree::end())
        )));
        assert!(trees.contains(&Tree::node(
            'x',
            Tree::node('x', Tree::end(), Tree::end()),
            Tree::node('x', Tree::end(), Tree::leaf('x'))
        )));
    }
}
