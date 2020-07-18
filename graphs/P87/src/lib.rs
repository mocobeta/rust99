use graph::Graph;
use std::hash::Hash;

pub fn nodes_by_depth_from<T: Hash + Copy + Eq + Ord>(g: &Graph<T>, start: T) -> Vec<T> {
    fn traverse<T: Hash + Copy + Eq + Ord>(g: &Graph<T>, v: T, visited: &mut Vec<T>) -> Vec<T> {
        if !visited.contains(&v) {
            visited.push(v);
        }
        let neighbors = g.get_node(&v).unwrap().adjacents();
        let to_visit: Vec<&T> = neighbors.iter().filter(|&n| !visited.contains(n)).collect();
        for n in to_visit {
            for w in traverse(g, *n, visited) {
                if !visited.contains(&w) {
                    visited.push(w);
                }
            }
        }
        visited.clone()
    }

    let mut res = traverse(g, start, &mut vec![]);
    res.reverse();
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    use P80::graph_converters::unlabeled;
    #[test]
    fn test_nodes_by_depth_from() {
        let g = unlabeled::from_string("[a-b, b-c, e, a-c, a-d]");
        let nodes = nodes_by_depth_from(&g, 'd');
        assert!(nodes == vec!['c', 'b', 'a', 'd'] || nodes == vec!['b', 'c', 'a', 'd']);
    }
}
