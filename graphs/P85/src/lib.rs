use graph::Graph;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

pub fn is_isomorphic_to<T: Hash + Copy + Eq, U: Hash + Copy + Eq>(
    g1: &Graph<T>,
    g2: &Graph<U>,
) -> bool {
    let unmapped1: Vec<T> = g1
        .get_nodes_by_degree(false)
        .iter()
        .map(|n| *n.get_value())
        .collect();
    let unmapped2: Vec<U> = g2
        .get_nodes_by_degree(false)
        .iter()
        .map(|n| *n.get_value())
        .collect();
    let mapping = HashMap::<T, U>::new();
    is_isonorphic_to_rec(g1, g2, &unmapped1, &unmapped2, &mapping)
}

fn is_isonorphic_to_rec<T: Hash + Copy + Eq, U: Hash + Copy + Eq>(
    g1: &Graph<T>,
    g2: &Graph<U>,
    unmapped1: &Vec<T>,
    unmapped2: &Vec<U>,
    mapping: &HashMap<T, U>,
) -> bool {
    if unmapped1.is_empty() {
        return true;
    }
    for i in 0..unmapped1.len() {
        for j in 0..unmapped2.len() {
            let mut unmapped1_copy = unmapped1.clone();
            let n1 = unmapped1_copy.remove(i);
            let mut unmapped2_copy = unmapped2.clone();
            let n2 = unmapped2_copy.remove(j);
            let mut mapping_copy = mapping.clone();
            mapping_copy.insert(n1, n2);
            if is_valid_mapping(g1, g2, &mapping_copy) {
                return is_isonorphic_to_rec(
                    g1,
                    g2,
                    &unmapped1_copy,
                    &unmapped2_copy,
                    &mapping_copy,
                );
            }
        }
    }
    return false;
}

fn is_valid_mapping<T: Hash + Copy + Eq, U: Hash + Copy + Eq>(
    g1: &Graph<T>,
    g2: &Graph<U>,
    mapping: &HashMap<T, U>,
) -> bool {
    for (n1, n2) in mapping {
        let node1 = g1.get_node(n1).unwrap();
        let node2 = g2.get_node(n2).unwrap();
        if node1.degree() != node2.degree() {
            // the degrees are not equivalent
            return false;
        } else {
            // check if the mapping is consistent
            let known_nodes1: Vec<&T> = mapping.keys().map(|x| x).collect();
            let known_nodes2: Vec<&U> = mapping.values().map(|x| x).collect();
            let adjs1: HashSet<T> = node1
                .adjacents()
                .iter()
                .filter(|n| known_nodes1.contains(n))
                .map(|n| *n)
                .collect();
            let adjs2: HashSet<U> = node2
                .adjacents()
                .iter()
                .filter(|n| known_nodes2.contains(n))
                .map(|n| *n)
                .collect();
            let ok = adjs1
                .iter()
                .all(|adj| adjs2.contains(mapping.get(adj).unwrap()));
            if !ok {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use P80::graph_converters::unlabeled;
    #[test]
    fn test_is_isomorphic_to() {
        let g1 = unlabeled::from_string("[a-b]");
        let g2 = unlabeled::from_string("[1-2]");
        assert!(is_isomorphic_to(&g1, &g2));

        let g1 = unlabeled::from_string("[a-b a-c b-d a-d]");
        let g2 = unlabeled::from_string("[1-3 2-3 3-4 2-4]");
        assert!(is_isomorphic_to(&g1, &g2));

        let g1 = unlabeled::from_string("[a-b a-c b-d a-d]");
        let g2 = unlabeled::from_string("[1-3 1-2 3-4 2-4]");
        assert!(!is_isomorphic_to(&g1, &g2));
    }
}
