use graph::Graph;
use P80::graph_converters::unlabeled;

pub fn spanning_trees(graph: &Graph<char>) -> Vec<Graph<char>> {
    let all_nodes: Vec<char> = graph.get_nodes().iter().map(|n| *n.get_value()).collect();
    if all_nodes.is_empty() {
        vec![]
    } else {
        // edges, visited
        let mut paths: Vec<(Vec<(char, char)>, Vec<char>)> = vec![];
        // resolved paths
        let mut found_paths: Vec<Vec<(char, char)>> = vec![];

        // initialize paths
        for start in all_nodes.clone() {
            paths.push((vec![], vec![start]));
        }
        // traverse graph
        while !paths.is_empty() {
            let (edges, visited) = paths.pop().unwrap();
            if visited.len() == graph.size() && !found_paths.contains(&edges) {
                found_paths.push(edges);
                continue;
            }
            for node in &visited {
                for next in graph.get_node(node).unwrap().adjacents() {
                    if visited.contains(&next) {
                        continue;
                    }
                    let mut new_edges = edges.clone();
                    let mut new_visited = visited.clone();
                    if *node < next {
                        new_edges.push((*node, next));
                    } else {
                        new_edges.push((next, *node));
                    }
                    new_edges.sort();
                    new_visited.push(next);
                    paths.push((new_edges, new_visited));
                }
            }
        }

        let mut trees = vec![];
        for path in found_paths {
            let tree = unlabeled::from_term_form(&all_nodes, &path);
            trees.push(tree);
        }
        trees
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_spanning_trees() {
        let g = unlabeled::from_string("[a-b, b-c, a-c]");
        let trees = spanning_trees(&g);
        assert_eq!(trees.len(), 3);
    }
}
