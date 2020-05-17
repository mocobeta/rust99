/// Multiway Tree
#[derive(Debug, Clone, PartialEq)]
pub struct MTree {
    value: char,
    children: Vec<MTree>,
}

impl MTree {
    /// Creates a leaf node
    pub fn leaf(value: char) -> Self {
        MTree::node(value, vec![])
    }
    /// Creates a node with children
    pub fn node(value: char, children: Vec<MTree>) -> Self {
        MTree {
            value: value,
            children: children,
        }
    }
    /// Returns the node value
    pub fn get_value(&self) -> char {
        self.value
    }
    /// Returns children
    pub fn get_children(&self) -> &Vec<MTree> {
        &self.children
    }
}
