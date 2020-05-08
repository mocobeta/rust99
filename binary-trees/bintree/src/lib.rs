use std::fmt;

/// Binary Tree
#[derive(Debug, Clone, PartialEq)]
pub enum Tree<T: fmt::Display> {
    Node {
        value: T,
        left: Box<Tree<T>>,
        right: Box<Tree<T>>,
    },
    End,
}

impl<T: fmt::Display> Tree<T> {
    pub fn end() -> Self {
        Tree::End
    }

    pub fn leaf(value: T) -> Self {
        Tree::node(value, Tree::End, Tree::End)
    }

    pub fn node(value: T, left: Tree<T>, right: Tree<T>) -> Self {
        Tree::Node {
            value: value,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    pub fn get_value(&self) -> Option<&T> {
        match self {
            Tree::Node { value, .. } => Some(value),
            Tree::End => None,
        }
    }

    pub fn get_left(&self) -> Option<&Tree<T>> {
        match self {
            Tree::Node {
                value: _,
                left,
                right: _,
            } => Some(left),
            Tree::End => None,
        }
    }

    pub fn get_right(&self) -> Option<&Tree<T>> {
        match self {
            Tree::Node {
                value: _,
                left: _,
                right,
            } => Some(right),
            Tree::End => None,
        }
    }

    pub fn get_left_mut(&mut self) -> Option<&mut Tree<T>> {
        match self {
            Tree::Node {
                value: _,
                left,
                right: _,
            } => Some(left),
            Tree::End => None,
        }
    }

    pub fn get_right_mut(&mut self) -> Option<&mut Tree<T>> {
        match self {
            Tree::Node {
                value: _,
                left: _,
                right,
            } => Some(right),
            Tree::End => None,
        }
    }

    pub fn replace_left(&mut self, t: Tree<T>) {
        if let Some(left) = self.get_left_mut() {
            *left = t;
        }
    }

    pub fn replace_right(&mut self, t: Tree<T>) {
        if let Some(right) = self.get_right_mut() {
            *right = t;
        }
    }

    pub fn count_nodes(&self) -> usize {
        match self {
            Tree::Node {
                value: _,
                left,
                right,
            } => 1 + left.count_nodes() + right.count_nodes(),
            Tree::End => 0,
        }
    }

    pub fn height(&self) -> usize {
        match self {
            Tree::Node {
                value: _,
                left,
                right,
            } => 1 + left.height().max(right.height()),
            Tree::End => 0,
        }
    }
}

impl<T: fmt::Display> fmt::Display for Tree<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Tree::Node { value, left, right } => write!(f, "T({} {} {})", value, left, right),
            Tree::End => write!(f, "."),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_end() {
        let tree = Tree::<char>::end();
        assert_eq!(tree, Tree::End);
    }

    #[test]
    fn test_leaf() {
        let tree = Tree::leaf('a');
        assert_eq!(
            tree,
            Tree::Node {
                value: 'a',
                left: Box::new(Tree::End),
                right: Box::new(Tree::End)
            }
        )
    }

    #[test]
    fn test_node() {
        let tree = Tree::node(
            'a',
            Tree::node('b', Tree::leaf('d'), Tree::leaf('e')),
            Tree::node(
                'c',
                Tree::end(),
                Tree::node('f', Tree::leaf('g'), Tree::end()),
            ),
        );
        assert_eq!(
            tree.to_string(),
            "T(a T(b T(d . .) T(e . .)) T(c . T(f T(g . .) .)))"
        );
    }

    #[test]
    fn test_get_value() {
        let tree = Tree::leaf('a');
        assert_eq!(tree.get_value(), Some(&'a'));
    }

    #[test]
    fn test_get_left() {
        let tree = Tree::node('a', Tree::leaf('b'), Tree::leaf('c'));
        assert_eq!(tree.get_left(), Some(&Tree::leaf('b')));
    }

    #[test]
    fn test_get_right() {
        let tree = Tree::node('a', Tree::leaf('b'), Tree::leaf('c'));
        assert_eq!(tree.get_right(), Some(&Tree::leaf('c')));
    }

    #[test]
    fn test_get_left_mut() {
        let mut tree = Tree::node('a', Tree::leaf('b'), Tree::leaf('c'));
        assert_eq!(tree.get_left_mut(), Some(&mut Tree::leaf('b')));
    }

    #[test]
    fn test_get_right_mut() {
        let mut tree = Tree::node('a', Tree::leaf('b'), Tree::leaf('c'));
        assert_eq!(tree.get_right_mut(), Some(&mut Tree::leaf('c')));
    }

    #[test]
    fn test_replace_left() {
        let mut tree = Tree::node('a', Tree::leaf('b'), Tree::leaf('c'));
        tree.replace_left(Tree::leaf('x'));
        assert_eq!(tree, Tree::node('a', Tree::leaf('x'), Tree::leaf('c')));
    }

    #[test]
    fn test_replace_right() {
        let mut tree = Tree::node('a', Tree::leaf('b'), Tree::leaf('c'));
        tree.replace_right(Tree::leaf('y'));
        assert_eq!(tree, Tree::node('a', Tree::leaf('b'), Tree::leaf('y')));
    }

    #[test]
    fn test_count_nodes() {
        let tree = Tree::<char>::end();
        assert_eq!(tree.count_nodes(), 0);

        let tree = Tree::leaf('a');
        assert_eq!(tree.count_nodes(), 1);

        let tree = Tree::node(
            'a',
            Tree::node('b', Tree::leaf('d'), Tree::leaf('e')),
            Tree::node(
                'c',
                Tree::end(),
                Tree::node('f', Tree::leaf('g'), Tree::end()),
            ),
        );
        assert_eq!(tree.count_nodes(), 7);
    }

    #[test]
    fn test_height() {
        let tree = Tree::<char>::end();
        assert_eq!(tree.height(), 0);

        let tree = Tree::leaf('a');
        assert_eq!(tree.height(), 1);

        let tree = Tree::node(
            'a',
            Tree::node('b', Tree::leaf('d'), Tree::leaf('e')),
            Tree::node(
                'c',
                Tree::end(),
                Tree::node('f', Tree::leaf('g'), Tree::end()),
            ),
        );
        assert_eq!(tree.height(), 4);
    }
}
