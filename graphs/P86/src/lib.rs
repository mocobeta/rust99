use graph::Graph;
use std::hash::Hash;

pub fn color_nodes<T: Hash + Copy + Eq>(g: &Graph<T>) -> Vec<(T, usize)> {
    let nodes = g.get_nodes_by_degree(true);
    let mut colored_nodes = vec![];
    let mut color = 0;
    let mut res = vec![];

    // Welsh-Powell's algorithm
    while colored_nodes.len() < g.size() {
        for node in &nodes {
            if colored_nodes.contains(node.get_value()) {
                continue;
            } else {
                color += 1;
                res.push((*node.get_value(), color));
                colored_nodes.push(*node.get_value());

                let neighbors = node.adjacents();
                for node2 in &nodes {
                    if !colored_nodes.contains(node2.get_value())
                        && !neighbors.contains(node2.get_value())
                    {
                        res.push((*node2.get_value(), color));
                        colored_nodes.push(*node2.get_value());
                    }
                }
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    use P80::graph_converters::unlabeled;
    #[test]
    fn test_color_nodes() {
        let g = unlabeled::from_string("[a-b, b-c, a-c, a-d]");
        let colored = color_nodes(&g);
        assert!(colored.contains(&('a', 1)));
        assert!(colored.contains(&('b', 2)));
        assert!(colored.contains(&('c', 3)));
        assert!(colored.contains(&('d', 2)));
    }
}
