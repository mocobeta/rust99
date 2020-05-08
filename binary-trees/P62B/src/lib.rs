use bintree::Tree;
use std::fmt;

pub fn at_level<T: Copy + fmt::Display>(tree: &Tree<T>, level: usize) -> Vec<T> {
    if level == 0 {
        panic!("level must be greater than 0");
    }
    if let Tree::Node { value, left, right } = tree {
        if level == 1 {
            vec![*value]
        } else {
            let mut vals = at_level(left, level - 1);
            vals.extend_from_slice(&at_level(right, level - 1));
            vals
        }
    } else {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_at_level() {
        assert_eq!(at_level(&Tree::<char>::end(), 1), vec![]);
        assert_eq!(at_level(&Tree::leaf('a'), 1), vec!['a']);
        assert_eq!(at_level(&Tree::leaf('a'), 2), vec![]);
        assert_eq!(
            at_level(
                &Tree::node(
                    'a',
                    Tree::leaf('b'),
                    Tree::node('c', Tree::leaf('d'), Tree::leaf('e'))
                ),
                2
            ),
            vec!['b', 'c']
        );
    }
}
