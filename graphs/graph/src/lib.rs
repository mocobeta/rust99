use std::cell::{Ref, RefCell};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::{Rc, Weak};

/// Graph node
#[derive(Debug, Clone)]
pub struct Node<T, U>
where
    T: Hash + Copy + Eq,
    U: Copy + Eq,
{
    value: T,
    edges: Vec<Weak<Edge<T, U>>>,
}

impl<T, U> Node<T, U>
where
    T: Hash + Copy + Eq,
    U: Copy + Eq,
{
    /// Returns node value
    pub fn get_value(&self) -> &T {
        &self.value
    }

    /// Returns adjacent node values
    pub fn adjacents(&self) -> Vec<T> {
        let mut adjs = vec![];
        self.edges.iter().for_each(|wref| {
            if let Some(edge) = wref.upgrade() {
                if let Some(v) = edge.get_adj(&self.value) {
                    adjs.push(v);
                }
            }
        });
        adjs
    }

    /// Returns adjacent node values with edge labels
    pub fn adjacents_with_label(&self) -> Vec<(T, Option<U>)> {
        let mut adjs = vec![];
        self.edges.iter().for_each(|wref| {
            if let Some(edge) = wref.upgrade() {
                if let Some(v) = edge.get_adj(&self.value) {
                    adjs.push((v, edge.get_label()));
                }
            }
        });
        adjs
    }

    /// Returns the degree of this node (num of adjacent nodes)
    pub fn degree(&self) -> usize {
        self.adjacents().len()
    }

    fn add_edge(&mut self, adj: Weak<Edge<T, U>>) {
        self.edges.push(adj);
    }
}

/// Graph Edge
#[derive(Debug, Clone)]
pub struct Edge<T, U>
where
    T: Hash + Copy + Eq,
    U: Copy + Eq,
{
    n1: Weak<RefCell<Node<T, U>>>,
    n2: Weak<RefCell<Node<T, U>>>,
    label: Option<U>,
}

impl<T, U> Edge<T, U>
where
    T: Hash + Copy + Eq,
    U: Copy + Eq,
{
    fn get_adj(&self, src: &T) -> Option<T> {
        if let Some(n1) = self.n1.upgrade() {
            if let Some(n2) = self.n2.upgrade() {
                if n1.borrow().get_value() == src {
                    Some(*n2.borrow().get_value())
                } else if n2.borrow().get_value() == src {
                    Some(*n1.borrow().get_value())
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    fn get_label(&self) -> Option<U> {
        self.label
    }
}

/// Undirected Labeled Graph
#[derive(Debug, Clone)]
pub struct LabeledGraph<T, U>
where
    T: Hash + Copy + Eq + PartialOrd,
    U: Copy + Eq,
{
    nodes: HashMap<T, Rc<RefCell<Node<T, U>>>>,
    edges: Vec<Rc<Edge<T, U>>>,
}

impl<T, U> LabeledGraph<T, U>
where
    T: Hash + Copy + Eq + Ord,
    U: Copy + Eq,
{
    /// Constructs an empty graph.
    pub fn new() -> Self {
        LabeledGraph {
            nodes: HashMap::new(),
            edges: vec![],
        }
    }

    /// Returns the number of nodes in this graph.
    pub fn size(&self) -> usize {
        self.nodes.len()
    }

    /// Returns a node with given value.
    pub fn get_node(&self, v: &T) -> Option<Ref<Node<T, U>>> {
        match self.nodes.get(v) {
            Some(node) => Some(node.borrow()),
            None => None,
        }
    }

    /// Returns all nodes in this graph.
    pub fn get_nodes(&self) -> Vec<Ref<Node<T, U>>> {
        self.nodes.iter().map(|(_, n)| n.borrow()).collect()
    }

    /// Returns all node vlues in thin graph.
    pub fn get_node_values(&self) -> Vec<T> {
        self.nodes.iter().map(|(&v, _)| v).collect()
    }

    /// Returns list of all nodes sorted according to decreasing (or increasing) degree.
    pub fn get_nodes_by_degree(&self, desc: bool) -> Vec<Ref<Node<T, U>>> {
        let mut nodes = self.get_nodes();
        if desc {
            nodes.sort_by(|a, b| {
                let ord = b.degree().cmp(&a.degree());
                match ord {
                    Ordering::Equal => a.get_value().cmp(&b.get_value()),
                    _ => ord,
                }
            });
        } else {
            nodes.sort_by(|a, b| {
                let ord = a.degree().cmp(&b.degree());
                match ord {
                    Ordering::Equal => a.get_value().cmp(&b.get_value()),
                    _ => ord,
                }
            });
        }
        nodes
    }

    /// Checks if two ndoes are linkedx
    pub fn is_linked(&self, v1: T, v2: T) -> bool {
        match &self.get_node(&v1) {
            Some(node) => node.adjacents().contains(&v2),
            None => false,
        }
    }

    /// Adds a node to this graph.
    pub fn add_node(&mut self, v: T) {
        let node = Node {
            value: v,
            edges: vec![],
        };
        self.nodes.insert(v, Rc::new(RefCell::new(node)));
    }

    /// Adds a labeled edge to this graph; i.e., connects two nodes in this graph.
    pub fn add_labeled_edge(&mut self, v1: T, v2: T, label: U) {
        self.add_edge_with_value(v1, v2, Some(label));
    }

    fn add_edge_with_value(&mut self, v1: T, v2: T, l: Option<U>) {
        let n1 = self.nodes.get(&v1).unwrap();
        let n2 = self.nodes.get(&v2).unwrap();
        let edge = Rc::new(Edge {
            n1: Rc::downgrade(n1),
            n2: Rc::downgrade(n2),
            label: l,
        });
        self.edges.push(edge);

        n1.borrow_mut()
            .add_edge(Rc::downgrade(self.edges.last().unwrap()));
        n2.borrow_mut()
            .add_edge(Rc::downgrade(self.edges.last().unwrap()));
    }
}

impl<T, U> PartialEq for LabeledGraph<T, U>
where
    T: Hash + Copy + Eq + Ord,
    U: Copy + Eq,
{
    fn eq(&self, other: &LabeledGraph<T, U>) -> bool {
        if self.size() != other.size() {
            return false;
        }
        for my_node in self.get_nodes() {
            if let None = other.get_node(my_node.get_value()) {
                return false;
            }
            let my_adjs: Vec<(T, Option<U>)> = my_node.adjacents_with_label().into_iter().collect();
            let other_adjs: Vec<(T, Option<U>)> = other
                .get_node(my_node.get_value())
                .unwrap()
                .adjacents_with_label()
                .into_iter()
                .collect();
            if my_adjs.len() != other_adjs.len() {
                return false;
            }
            for adj in my_adjs {
                if !other_adjs.contains(&adj) {
                    return false;
                }
            }
        }
        true
    }
}

/// Undirected Graph
#[allow(dead_code)]
pub type Graph<T> = LabeledGraph<T, ()>;

impl<T: Hash + Copy + Eq + Ord> Graph<T> {
    /// Adds an edge to this graph; i.e., connects two nodes in this graph.
    pub fn add_edge(&mut self, v1: T, v2: T) {
        self.add_edge_with_value(v1, v2, None);
    }
}

/// Directed Labeled Graph
#[derive(Debug, Clone)]
pub struct LabeledDigraph<T, U>
where
    T: Hash + Copy + Eq,
    U: Copy + Eq,
{
    nodes: HashMap<T, Rc<RefCell<Node<T, U>>>>,
    edges: Vec<Rc<Edge<T, U>>>,
}

impl<T, U> LabeledDigraph<T, U>
where
    T: Hash + Copy + Eq,
    U: Copy + Eq,
{
    /// Constructs an empty digraph.
    pub fn new() -> Self {
        LabeledDigraph {
            nodes: HashMap::new(),
            edges: vec![],
        }
    }

    /// Returns the number of nodes in this digraph.
    pub fn size(&self) -> usize {
        self.nodes.len()
    }

    /// Returns a node with given value.
    pub fn get_node(&self, v: &T) -> Option<Ref<Node<T, U>>> {
        match self.nodes.get(v) {
            Some(node) => Some(node.borrow()),
            None => None,
        }
    }

    /// Returns all nodes in this digraph.
    pub fn get_nodes(&self) -> Vec<Ref<Node<T, U>>> {
        self.nodes.iter().map(|(_, n)| n.borrow()).collect()
    }

    /// Returns all node vlues in thin graph.
    pub fn get_node_values(&self) -> Vec<T> {
        self.nodes.iter().map(|(&v, _)| v).collect()
    }

    /// Adds a node to this digraph.
    pub fn add_node(&mut self, v: T) {
        let node = Node {
            value: v,
            edges: vec![],
        };
        self.nodes.insert(v, Rc::new(RefCell::new(node)));
    }

    /// Adds a labeled arc to this digraph; i.e., connects two nodes in this graph.
    pub fn add_labeled_arc(&mut self, src: T, dst: T, label: U) {
        self.add_arc_with_value(src, dst, Some(label));
    }

    fn add_arc_with_value(&mut self, src: T, dst: T, l: Option<U>) {
        let n1 = self.nodes.get(&src).unwrap();
        let n2 = self.nodes.get(&dst).unwrap();
        let edge = Rc::new(Edge {
            n1: Rc::downgrade(n1),
            n2: Rc::downgrade(n2),
            label: l,
        });
        self.edges.push(edge);

        n1.borrow_mut()
            .add_edge(Rc::downgrade(self.edges.last().unwrap()));
    }
}

/// Undirected Diraph
#[allow(dead_code)]
pub type Digraph<T> = LabeledDigraph<T, ()>;

impl<T: Hash + Copy + Eq> Digraph<T> {
    /// Adds an arc to this graph; i.e., connects two nodes in this graph.
    pub fn add_arc(&mut self, src: T, dst: T) {
        self.add_arc_with_value(src, dst, None);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_graph() {
        let mut g = Graph::<char>::new();
        g.add_node('a');
        g.add_node('b');
        g.add_node('c');
        g.add_edge('a', 'b');
        g.add_edge('b', 'c');
        g.add_edge('c', 'a');

        assert_eq!(g.size(), 3);
        let n1 = g.get_node(&'a').unwrap();
        let n2 = g.get_node(&'b').unwrap();
        let n3 = g.get_node(&'c').unwrap();
        assert_eq!(n1.get_value(), &'a');
        assert_eq!(n2.get_value(), &'b');
        assert_eq!(n3.get_value(), &'c');
        assert_eq!(n1.adjacents(), vec!['b', 'c']);
        assert_eq!(n2.adjacents(), vec!['a', 'c']);
        assert_eq!(n3.adjacents(), vec!['b', 'a']);
    }

    #[test]
    fn test_construct_labeled_graph() {
        let mut g = LabeledGraph::<char, i32>::new();
        g.add_node('a');
        g.add_node('b');
        g.add_node('c');
        g.add_labeled_edge('a', 'b', 2);
        g.add_labeled_edge('b', 'c', 1);
        g.add_labeled_edge('c', 'a', 3);

        assert_eq!(g.size(), 3);
        let n1 = g.get_node(&'a').unwrap();
        let n2 = g.get_node(&'b').unwrap();
        let n3 = g.get_node(&'c').unwrap();
        assert_eq!(n1.get_value(), &'a');
        assert_eq!(n2.get_value(), &'b');
        assert_eq!(n3.get_value(), &'c');
        assert_eq!(
            n1.adjacents_with_label(),
            vec![('b', Some(2)), ('c', Some(3))]
        );
        assert_eq!(
            n2.adjacents_with_label(),
            vec![('a', Some(2)), ('c', Some(1))]
        );
        assert_eq!(
            n3.adjacents_with_label(),
            vec![('b', Some(1)), ('a', Some(3))]
        );
    }

    #[test]
    fn test_construct_digraph() {
        let mut g = Digraph::<char>::new();
        g.add_node('a');
        g.add_node('b');
        g.add_node('c');
        g.add_arc('a', 'b');
        g.add_arc('a', 'c');
        g.add_arc('b', 'c');
        g.add_arc('c', 'a');

        assert_eq!(g.size(), 3);
        let n1 = g.get_node(&'a').unwrap();
        let n2 = g.get_node(&'b').unwrap();
        let n3 = g.get_node(&'c').unwrap();
        assert_eq!(n1.get_value(), &'a');
        assert_eq!(n2.get_value(), &'b');
        assert_eq!(n3.get_value(), &'c');
        assert_eq!(n1.adjacents(), vec!['b', 'c']);
        assert_eq!(n2.adjacents(), vec!['c']);
        assert_eq!(n3.adjacents(), vec!['a']);
    }

    #[test]
    fn test_construct_labeled_digraph() {
        let mut g = LabeledDigraph::<char, i32>::new();
        g.add_node('a');
        g.add_node('b');
        g.add_node('c');
        g.add_labeled_arc('a', 'b', 2);
        g.add_labeled_arc('a', 'c', 3);
        g.add_labeled_arc('b', 'c', 1);
        g.add_labeled_arc('c', 'a', 3);

        assert_eq!(g.size(), 3);
        let n1 = g.get_node(&'a').unwrap();
        let n2 = g.get_node(&'b').unwrap();
        let n3 = g.get_node(&'c').unwrap();
        assert_eq!(n1.get_value(), &'a');
        assert_eq!(n2.get_value(), &'b');
        assert_eq!(n3.get_value(), &'c');
        assert_eq!(
            n1.adjacents_with_label(),
            vec![('b', Some(2)), ('c', Some(3))]
        );
        assert_eq!(n2.adjacents_with_label(), vec![('c', Some(1))]);
        assert_eq!(n3.adjacents_with_label(), vec![('a', Some(3))]);
    }

    #[test]
    fn test_degree() {
        let mut g = Graph::<char>::new();
        g.add_node('a');
        g.add_node('b');
        g.add_node('c');
        g.add_node('d');
        g.add_edge('a', 'b');
        g.add_edge('b', 'c');
        g.add_edge('a', 'c');
        g.add_edge('a', 'd');

        assert_eq!(g.get_node(&'a').unwrap().degree(), 3);
    }

    #[test]
    fn test_nodes_by_degree() {
        let mut g = Graph::<char>::new();
        g.add_node('a');
        g.add_node('b');
        g.add_node('c');
        g.add_node('d');
        g.add_edge('a', 'b');
        g.add_edge('b', 'c');
        g.add_edge('a', 'c');
        g.add_edge('a', 'd');

        let nodes = g.get_nodes_by_degree(true);
        assert_eq!(nodes.get(0).unwrap().get_value(), &'a');
        assert_eq!(nodes.get(1).unwrap().get_value(), &'b');
        assert_eq!(nodes.get(2).unwrap().get_value(), &'c');
        assert_eq!(nodes.get(3).unwrap().get_value(), &'d');
    }
}
