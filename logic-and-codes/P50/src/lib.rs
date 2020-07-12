use std::cmp::Ordering;
use std::collections::BinaryHeap;

pub fn huffman(symbols: &Vec<(char, usize)>) -> Vec<(char, String)> {
    // initialize min heap
    let mut min_heap = BinaryHeap::new();
    for &(s, f) in symbols {
        min_heap.push(Node::Leaf(f, s));
    }
    // build huffman tree from the min heap
    let huffman_tree = build_huffman_tree(&mut min_heap);
    // convert huffman tree to code
    huffmann_tree_to_code(&huffman_tree.unwrap())
}

fn build_huffman_tree(min_heap: &mut BinaryHeap<Node>) -> Option<Node> {
    while min_heap.len() > 1 {
        let left = min_heap.pop().unwrap();
        let right = min_heap.pop().unwrap();
        let new_node = Node::Internal(left.freq() + right.freq(), Box::new(left), Box::new(right));
        min_heap.push(new_node);
    }
    min_heap.pop()
}

fn huffmann_tree_to_code(root: &Node) -> Vec<(char, String)> {
    fn generate_huffman_codes(node: &Node, buf: String) -> Vec<(char, String)> {
        match node {
            Node::Leaf(_, symbol) => vec![(*symbol, buf)],
            Node::Internal(_, left, right) => {
                let mut res = vec![];
                let mut buf_left = buf.clone();
                buf_left.push('0');
                res.extend(generate_huffman_codes(&left, buf_left));
                let mut buf_right = buf.clone();
                buf_right.push('1');
                res.extend(generate_huffman_codes(&right, buf_right));
                res
            }
        }
    }

    let mut code = generate_huffman_codes(root, String::new());
    code.sort_by(|a, b| a.0.cmp(&b.0));
    code
}

#[derive(Debug)]
/// Reresents a leaf or internal node of Hufmann tree
enum Node {
    Leaf(usize, char),
    Internal(usize, Box<Node>, Box<Node>),
}

impl Node {
    fn freq(&self) -> usize {
        match self {
            Node::Leaf(freq, _) => *freq,
            Node::Internal(freq, _, _) => *freq,
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.freq() == other.freq()
    }
}

impl Eq for Node {}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.freq().cmp(&other.freq()) {
            Ordering::Equal => Ordering::Equal,
            Ordering::Greater => Ordering::Less,
            Ordering::Less => Ordering::Greater,
        }
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_node_min_heap() {
        let mut heap = BinaryHeap::new();
        heap.push(Node::Leaf(5, 'a'));
        heap.push(Node::Leaf(10, 'b'));
        heap.push(Node::Internal(
            8,
            Box::new(Node::Leaf(4, 'c')),
            Box::new(Node::Leaf(4, 'd')),
        ));

        assert_eq!(heap.pop(), Some(Node::Leaf(5, 'a')));
        assert_eq!(
            heap.pop(),
            Some(Node::Internal(
                8,
                Box::new(Node::Leaf(4, 'c')),
                Box::new(Node::Leaf(4, 'd'))
            ))
        );
        assert_eq!(heap.pop(), Some(Node::Leaf(10, 'b')));
    }

    #[test]
    fn test_build_huffman_tree() {
        let mut heap = BinaryHeap::new();
        heap.push(Node::Leaf(45, 'a'));
        heap.push(Node::Leaf(13, 'b'));
        heap.push(Node::Leaf(12, 'c'));
        heap.push(Node::Leaf(16, 'd'));
        heap.push(Node::Leaf(9, 'e'));
        heap.push(Node::Leaf(5, 'f'));
        let huffman_tree = build_huffman_tree(&mut heap);
        assert_eq!(
            huffman_tree.unwrap(),
            Node::Internal(
                100,
                Box::new(Node::Leaf(45, 'a')),
                Box::new(Node::Internal(
                    55,
                    Box::new(Node::Internal(
                        25,
                        Box::new(Node::Leaf(12, 'c')),
                        Box::new(Node::Leaf(13, 'b'))
                    )),
                    Box::new(Node::Internal(
                        30,
                        Box::new(Node::Internal(
                            14,
                            Box::new(Node::Leaf(5, 'f')),
                            Box::new(Node::Leaf(9, 'e'))
                        )),
                        Box::new(Node::Leaf(16, 'd'))
                    ))
                ))
            )
        );
    }

    #[test]
    fn test_huffman_tree_to_code() {
        let root = Node::Internal(
            30,
            Box::new(Node::Internal(
                14,
                Box::new(Node::Leaf(5, 'c')),
                Box::new(Node::Leaf(9, 'b')),
            )),
            Box::new(Node::Leaf(16, 'a')),
        );
        let huffman_code = huffmann_tree_to_code(&root);
        assert_eq!(
            huffman_code,
            vec![
                ('a', "1".to_string()),
                ('b', "01".to_string()),
                ('c', "00".to_string()),
            ]
        );
    }

    #[test]
    fn test_huffman() {
        let symbols = vec![
            ('a', 45),
            ('b', 13),
            ('c', 12),
            ('d', 16),
            ('e', 9),
            ('f', 5),
        ];
        assert_eq!(
            huffman(&symbols),
            vec![
                ('a', "0".to_string()),
                ('b', "101".to_string()),
                ('c', "100".to_string()),
                ('d', "111".to_string()),
                ('e', "1101".to_string()),
                ('f', "1100".to_string())
            ]
        );
    }
}
