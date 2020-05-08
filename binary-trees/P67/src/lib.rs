#![feature(or_patterns)]
use std::fmt;

/// Binary Tree
#[derive(Debug, Clone, PartialEq)]
pub enum Tree {
    Node {
        value: char,
        left: Box<Tree>,
        right: Box<Tree>,
    },
    End,
}

impl fmt::Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Tree::Node { value, left, right } => match (left.as_ref(), right.as_ref()) {
                (Tree::End, Tree::End) => write!(f, "{}", value),
                _ => write!(f, "{}({},{})", value, left, right),
            },
            Tree::End => write!(f, ""),
        }
    }
}

/// Utility methods for Tree
impl Tree {
    pub fn end() -> Self {
        Tree::End
    }

    pub fn leaf(value: char) -> Self {
        Tree::node(value, Tree::End, Tree::End)
    }

    pub fn node(value: char, left: Tree, right: Tree) -> Self {
        Tree::Node {
            value: value,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    pub fn get_left(&self) -> Option<&Tree> {
        match self {
            Tree::Node {
                value: _,
                left,
                right: _,
            } => Some(left),
            Tree::End => None,
        }
    }

    pub fn get_right(&self) -> Option<&Tree> {
        match self {
            Tree::Node {
                value: _,
                left: _,
                right,
            } => Some(right),
            Tree::End => None,
        }
    }

    pub fn get_left_mut(&mut self) -> Option<&mut Tree> {
        match self {
            Tree::Node {
                value: _,
                left,
                right: _,
            } => Some(left),
            Tree::End => None,
        }
    }

    pub fn get_right_mut(&mut self) -> Option<&mut Tree> {
        match self {
            Tree::Node {
                value: _,
                left: _,
                right,
            } => Some(right),
            Tree::End => None,
        }
    }

    pub fn replace_left(&mut self, t: Tree) {
        if let Some(left) = self.get_left_mut() {
            *left = t;
        }
    }

    pub fn replace_right(&mut self, t: Tree) {
        if let Some(right) = self.get_right_mut() {
            *right = t;
        }
    }
}

impl Tree {
    pub fn from_string(s: &str) -> Tree {
        if s.len() == 0 {
            Tree::end()
        } else {
            match Tree::parse(s) {
                (tree, "") => tree,
                _ => panic!(),
            }
        }
    }
    fn parse(s: &str) -> (Tree, &str) {
        let mut value = None;
        let mut left = Tree::end();
        let mut right = Tree::end();
        let mut tail = s;
        let mut state = State::StartValue;
        while tail.len() > 0 {
            let (c, _tail) = tail.split_at(1);
            match (c, &mut state) {
                ("(", State::StartLeft) => {
                    let (l, t) = Tree::parse(_tail);
                    left = l;
                    tail = t;
                    state = State::StartRight;
                }
                (")", State::StartLeft) => {
                    tail = _tail;
                    break;
                }
                (")", State::StartValue | State::StartRight | State::EndRight) => {
                    tail = _tail;
                    break;
                }
                (",", State::StartLeft | State::StartValue | State::EndRight) => {
                    break;
                }
                (",", State::StartRight) => {
                    let (r, t) = Tree::parse(_tail);
                    right = r;
                    tail = t;
                    state = State::EndRight;
                }
                (_, State::StartValue) => {
                    value = c.chars().nth(0);
                    tail = _tail;
                    state = State::StartLeft;
                }
                _ => panic!("Unaccepted input! c={} state={:?}", c, state),
            }
        }
        match value {
            Some(v) => (Tree::node(v, left, right), tail),
            None => (Tree::end(), tail),
        }
    }
}

#[derive(Debug)]
enum State {
    StartValue,
    StartLeft,
    StartRight,
    EndRight,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_string() {
        let tree = Tree::end();
        assert_eq!(tree.to_string(), "");

        let tree = Tree::leaf('a');
        assert_eq!(tree.to_string(), "a");

        let tree = Tree::node('a', Tree::leaf('b'), Tree::end());
        assert_eq!(tree.to_string(), "a(b,)");

        let tree = Tree::node('a', Tree::end(), Tree::leaf('c'));
        assert_eq!(tree.to_string(), "a(,c)");

        let tree = Tree::node('a', Tree::leaf('b'), Tree::leaf('c'));
        assert_eq!(tree.to_string(), "a(b,c)");
    }

    #[test]
    fn test_from_string() {
        let tree = Tree::from_string("");
        assert_eq!(tree, Tree::end());

        let tree = Tree::from_string("a");
        assert_eq!(tree, Tree::leaf('a'));

        let tree = Tree::from_string("a(b,c)");
        assert_eq!(tree, Tree::node('a', Tree::leaf('b'), Tree::leaf('c')));

        let tree = Tree::from_string("a(b,)");
        assert_eq!(tree, Tree::node('a', Tree::leaf('b'), Tree::end()));

        let tree = Tree::from_string("a(,c)");
        assert_eq!(tree, Tree::node('a', Tree::end(), Tree::leaf('c')));

        let tree = Tree::from_string("a(b(d,e),c(,f(g,)))");
        assert_eq!(
            tree,
            Tree::node(
                'a',
                Tree::node('b', Tree::leaf('d'), Tree::leaf('e')),
                Tree::node(
                    'c',
                    Tree::end(),
                    Tree::node('f', Tree::leaf('g'), Tree::end()),
                )
            )
        );
    }
}
