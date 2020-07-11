use graph::LabeledGraph;
use P80::graph_converters::labeled;

pub fn minimal_spanning_trees(graph: &LabeledGraph<char, i32>) -> Vec<LabeledGraph<char, i32>> {
    let all_nodes: Vec<char> = graph.get_nodes().iter().map(|n| *n.get_value()).collect();
    if all_nodes.is_empty() {
        vec![]
    } else {
        // edges, frontiers, visited
        let mut paths: Vec<(Vec<(char, char, i32)>, Vec<char>)> = vec![];
        // resolved minimum paths
        let mut found_paths: Vec<(Vec<(char, char, i32)>, i32)> = vec![];

        let path_score = |edges: &Vec<(char, char, i32)>| edges.iter().map(|(_, _, l)| l).sum();

        // initialize paths
        for start in all_nodes.clone() {
            paths.push((vec![], vec![start]));
        }
        // traverse graph
        while !paths.is_empty() {
            let (edges, visited) = paths.pop().unwrap();
            let score = path_score(&edges);
            if visited.len() == graph.size() {
                found_paths.push((edges, score));
                continue;
            }
            let mut min_edge = ('x', 'x', std::i32::MAX);
            let mut found = false;
            for node in &visited {
                for (next, label) in graph.get_node(node).unwrap().adjacents_with_label() {
                    if visited.contains(&next) {
                        continue;
                    }
                    if label.unwrap() < min_edge.2 {
                        min_edge = (*node, next, label.unwrap());
                        found = true;
                    }
                }
            }
            if found {
                let mut new_edges = edges.clone();
                new_edges.push(min_edge);
                let mut new_visited = visited.clone();
                new_visited.push(min_edge.1);
                paths.push((new_edges, new_visited));
            }
        }

        let mut trees = vec![];
        for (path, _) in found_paths {
            let tree = labeled::from_term_form(&all_nodes, &path);
            if !trees.contains(&tree) {
                trees.push(tree);
            }
        }
        trees
    }
}

pub fn label_sum(graph: &LabeledGraph<char, i32>) -> i32 {
    labeled::to_term_form(&graph)
        .1
        .iter()
        .map(|(_, _, l)| l)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_spanning_trees() {
        let g = labeled::from_string("[a-b/1, b-c/2, a-c/3]");
        let trees = minimal_spanning_trees(&g);
        assert_eq!(trees.len(), 1);
        assert_eq!(label_sum(&trees.get(0).unwrap()), 3);
    }
}
