use graph::Graph;
use std::hash::Hash;
use P87::nodes_by_depth_from;

pub fn split_graph<T: Hash + Copy + Eq + Ord>(g: &Graph<T>) -> Vec<Graph<T>> {
    let mut paths = Vec::<Vec<T>>::new();
    let node_vals = g.get_node_values();

    // find all depth-first paths
    for v in node_vals {
        let is_visited = paths.iter().any(|path| path.contains(&v));
        if !is_visited {
            let path = nodes_by_depth_from(&g, v);
            paths.push(path);
        }
    }

    // make partial graphs from the found paths
    let mut res = vec![];
    for path in paths {
        let mut part_graph = Graph::<T>::new();

        // register nodes
        for v in &path {
            part_graph.add_node(*v);
        }

        // register edges
        for v in &path {
            for adj in g.get_node(&v).unwrap().adjacents() {
                if !part_graph.is_linked(*v, adj) {
                    part_graph.add_edge(*v, adj);
                }
            }
        }

        res.push(part_graph);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    use P80::graph_converters::unlabeled;

    #[test]
    fn test_split_graph() {
        assert_eq!(split_graph(&Graph::<char>::new()), vec![]);
        assert_eq!(
            split_graph(&unlabeled::from_string("[a-b]")),
            vec![unlabeled::from_string("[a-b]")]
        );

        let actual = split_graph(&unlabeled::from_string("[a-b c]"));
        let expected = vec![
            unlabeled::from_string("[a-b]"),
            unlabeled::from_string("[c]"),
        ];
        assert!(expected.iter().all(|g| actual.contains(&g)));

        let actual = split_graph(&unlabeled::from_string("[a-b, b-c, d, e-f, f-g, g-e, h]"));
        let expected = vec![
            unlabeled::from_string("[a-b b-c]"),
            unlabeled::from_string("[d]"),
            unlabeled::from_string("[e-f f-g g-e]"),
            unlabeled::from_string("[h]"),
        ];
        assert!(expected.iter().all(|g| actual.contains(&g)));
    }
}
