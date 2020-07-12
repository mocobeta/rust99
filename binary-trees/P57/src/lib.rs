use bintree::Tree;
use std::fmt;

pub fn add_value<T: fmt::Display + Copy + PartialOrd>(node: &mut Tree<T>, value: T) {
    if let Some(v) = node.get_value() {
        if value < *v {
            if let Some(left) = node.get_left_mut() {
                add_value(left, value);
            }
        } else {
            if let Some(right) = node.get_right_mut() {
                add_value(right, value);
            }
        }
    } else {
        *node = Tree::leaf(value);
    }
}

pub fn from_list<T: fmt::Display + Copy + PartialOrd>(values: &Vec<T>) -> Tree<T> {
    let mut tree = Tree::<T>::end();
    for value in values {
        add_value(&mut tree, *value);
    }
    tree
}

#[cfg(test)]
mod tests {
    use super::*;
    use bintree::Tree;

    #[test]
    fn test_add_value() {
        let mut tree = Tree::end();
        add_value(&mut tree, 2);
        assert_eq!(tree.to_string(), "T(2 . .)");

        add_value(&mut tree, 3);
        assert_eq!(tree.to_string(), "T(2 . T(3 . .))");

        add_value(&mut tree, 0);
        assert_eq!(tree.to_string(), "T(2 T(0 . .) T(3 . .))");
    }

    #[test]
    fn test_from_list() {
        let tree = from_list(&vec![3, 2, 5, 7, 1]);
        assert_eq!(tree.to_string(), "T(3 T(2 T(1 . .) .) T(5 . T(7 . .)))")
    }
}
