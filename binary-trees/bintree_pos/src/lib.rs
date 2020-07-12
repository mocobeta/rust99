use std::fmt;

/// Positioned Tree; a Tree with position (x, y)
#[derive(Debug, Clone, PartialEq)]
pub enum PositionedTree<T: fmt::Display> {
    Node {
        value: T,
        left: Box<PositionedTree<T>>,
        right: Box<PositionedTree<T>>,
        x: u32,
        y: u32,
    },
    End,
}

impl<T: fmt::Display> PositionedTree<T> {
    pub fn end() -> Self {
        PositionedTree::End
    }
    pub fn leaf(v: T, x: u32, y: u32) -> Self {
        PositionedTree::node(v, PositionedTree::End, PositionedTree::End, x, y)
    }
    pub fn node(v: T, left: PositionedTree<T>, right: PositionedTree<T>, x: u32, y: u32) -> Self {
        PositionedTree::Node {
            value: v,
            left: Box::new(left),
            right: Box::new(right),
            x: x,
            y: y,
        }
    }
    pub fn get_x(&self) -> Option<u32> {
        match self {
            PositionedTree::Node {
                value: _,
                left: _,
                right: _,
                x,
                y: _,
            } => Some(*x),
            PositionedTree::End => None,
        }
    }
    pub fn get_y(&self) -> Option<u32> {
        match self {
            PositionedTree::Node {
                value: _,
                left: _,
                right: _,
                x: _,
                y,
            } => Some(*y),
            PositionedTree::End => None,
        }
    }
}

impl<T: fmt::Display> fmt::Display for PositionedTree<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            PositionedTree::Node {
                value,
                left,
                right,
                x,
                y,
            } => write!(f, "T[{},{}]({} {} {})", x, y, value, left, right),
            PositionedTree::End => write!(f, "."),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positioned_node() {
        let tree = PositionedTree::node(
            'a',
            PositionedTree::node(
                'b',
                PositionedTree::end(),
                PositionedTree::leaf('c', 2, 3),
                1,
                2,
            ),
            PositionedTree::leaf('d', 4, 2),
            3,
            1,
        );
        assert_eq!(
            tree.to_string(),
            "T[3,1](a T[1,2](b . T[2,3](c . .)) T[4,2](d . .))"
        )
    }
}
