use graph::Graph;
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;
use P88::split_graph;

pub fn is_bipartite<T: Hash + Copy + Eq + Ord>(g: &Graph<T>) -> bool {
    assert!(g.size() > 0, "graph must not be empty!");

    let mut colored = HashMap::<i8, HashSet<T>>::new();
    colored.insert(1, HashSet::<T>::new());
    colored.insert(-1, HashSet::<T>::new());

    let mut queue = vec![];

    // add first node of all splitted graphs to side 1
    let sub_graphs = split_graph(&g);
    for sub_graph in sub_graphs {
        let v = sub_graph.get_node_values()[0];
        queue.push(v);
        colored.get_mut(&1i8).unwrap().insert(v);
    }

    while !queue.is_empty() {
        // pop next node to visit
        let v = queue.pop().unwrap();

        // check if v is already colored
        assert!(
            colored.get(&1i8).unwrap().contains(&v) || colored.get(&-1i8).unwrap().contains(&v)
        );

        // identify the color of current node
        let color = if colored.get(&1i8).unwrap().contains(&v) {
            1
        } else {
            -1
        };

        let neighbors = g.get_node(&v).unwrap().adjacents();
        for neighbor in neighbors {
            if colored.get(&color).unwrap().contains(&neighbor) {
                // neighbor is on the same side. the graph is not a bipartite.
                return false;
            } else if colored.get(&(color * -1)).unwrap().contains(&neighbor) {
                // nothing to do
            } else {
                // found unvisited node
                colored.get_mut(&(color * -1)).unwrap().insert(neighbor);
                queue.push(neighbor);
            }
        }
    }
    true
}

#[cfg(test)]
mod tests_undirected {
    use super::*;
    use P80::graph_converters::unlabeled;

    #[test]
    fn test_is_bipartite() {
        assert_eq!(
            is_bipartite(&unlabeled::from_string("[a-b, b-c, c-a]")),
            false
        );

        assert_eq!(is_bipartite(&unlabeled::from_string("[a-b, b-c, d]")), true);

        assert_eq!(
            is_bipartite(&unlabeled::from_string("[a-b, b-c, d, e-f, f-g, g-e, h]")),
            false
        );

        assert_eq!(
            is_bipartite(&unlabeled::from_string("[a-b, b-c, d, e-f, g-e, h]")),
            true
        );
    }
}
