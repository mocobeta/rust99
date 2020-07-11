use graph::{Graph, Node};
use std::hash::Hash;

pub fn nodes_by_depth_from<T: Hash + Copy + Eq>(g: &Graph<T>, start: T) -> Vec<T> {
    traverse(g, start, &mut vec![])
}

fn traverse<T: Hash + Copy + Eq>(g: &Graph<T>, v: T, visited: &mut Vec<T>) -> Vec<T> {
    let neighbors = g.get_node(&v).unwrap().adjacents();
    if neighbors.iter().all(|n| visited.contains(n)) {
        vec![v]
    } else {
        visited.push(v);
        let mut res = vec![];
        for n in neighbors {
            if !visited.contains(&n) {
                res.extend_from_slice(&traverse(g, n, visited));
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use P80::graph_converters::unlabeled;
    #[test]
    fn test_nodes_by_depth_from() {
        let g = unlabeled::from_string("[a-b, b-c, e, a-c, a-d]");
        assert_eq!(nodes_by_depth_from(&g, 'd'), vec!['c', 'b', 'a', 'd']);
    }
}
