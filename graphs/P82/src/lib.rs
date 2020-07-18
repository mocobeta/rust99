use graph::{LabeledDigraph, LabeledGraph};

pub trait CycleFinder<T> {
    fn find_cycles(&self, node: char) -> Vec<Vec<char>>;

    fn find_all_cycles<F>(node: char, adjacents: F) -> Vec<Vec<char>>
    where
        F: Fn(char) -> Vec<char>,
    {
        let mut paths = vec![];
        let mut cycles = vec![];

        // add the start node to visited list and paths.
        paths.push(vec![node]);

        // repeats until all edges are marked visited.
        while !paths.is_empty() {
            let path = paths.pop().unwrap();
            let last = *path.last().unwrap();
            let adjs = adjacents(last);
            for next in adjs {
                if path.len() > 1 && *path.get(path.len() - 2).unwrap() == next {
                    continue; // prevent bouncing
                }
                let mut new_path = path.clone();
                new_path.push(next);
                if next == node {
                    // reached to the end
                    cycles.push(new_path);
                } else {
                    // continue to traverse the graph
                    paths.push(new_path);
                }
            }
        }
        cycles
    }
}

impl<U: Copy + Eq> CycleFinder<char> for LabeledGraph<char, U> {
    fn find_cycles(&self, node: char) -> Vec<Vec<char>> {
        let adjacents = |v| self.get_node(&v).unwrap().adjacents();
        if let Some(_) = self.get_node(&node) {
            Self::find_all_cycles(node, adjacents)
        } else {
            vec![]
        }
    }
}

impl<U: Copy + Eq> CycleFinder<char> for LabeledDigraph<char, U> {
    fn find_cycles(&self, node: char) -> Vec<Vec<char>> {
        let adjacents = |v| self.get_node(&v).unwrap().adjacents();
        if let Some(_) = self.get_node(&node) {
            Self::find_all_cycles(node, adjacents)
        } else {
            vec![]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use std::hash::Hash;

    #[test]
    fn test_find_cycles_graph() {
        use P80::graph_converters::unlabeled;
        let g = unlabeled::from_string("[b-c, f-c, g-h, d, f-b, k-f, h-g]");
        assert!(has_same_elements(
            &g.find_cycles('f'),
            &vec![vec!['f', 'c', 'b', 'f'], vec!['f', 'b', 'c', 'f']]
        ));
        assert_eq!(g.find_cycles('g'), Vec::<Vec<char>>::new());
    }

    #[test]
    fn test_find_cycles_digraph() {
        use P80::digraph_converters::unlabeled;
        let g = unlabeled::from_string("[b>c, c>b, c>f, f>c, g>h, d, b>f, f>b, k>f, h>g]");
        assert!(has_same_elements(
            &g.find_cycles('f'),
            &vec![vec!['f', 'c', 'b', 'f'], vec!['f', 'b', 'c', 'f']]
        ));
        assert_eq!(g.find_cycles('g'), Vec::<Vec<char>>::new());
    }

    fn has_same_elements<T: Hash + Eq>(li1: &Vec<T>, li2: &Vec<T>) -> bool {
        if li1.len() != li2.len() {
            false
        } else {
            let set1: HashSet<&T> = li1.iter().collect();
            let set2: HashSet<&T> = li2.iter().collect();
            set1 == set2
        }
    }
}

/*
#[allow(dead_code)]
mod test_util {
    use std::collections::HashSet;
    use std::hash::Hash;
    pub fn has_same_elements<T: Hash + Eq>(li1: &Vec<T>, li2: &Vec<T>) -> bool {
        if li1.len() != li2.len() {
            false
        } else {
            let set1: HashSet<&T> = li1.iter().collect();
            let set2: HashSet<&T> = li2.iter().collect();
            set1 == set2
        }
    }
}
*/
