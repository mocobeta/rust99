use graph::{LabeledDigraph, LabeledGraph};

pub trait PathFinder<T> {
    fn find_paths(&self, start: char, end: char) -> Vec<Vec<T>>;

    fn find_all_paths<F>(start: char, end: char, adjacents: F) -> Vec<Vec<char>>
    where
        F: Fn(char) -> Vec<char>,
    {
        let mut paths = vec![];
        let mut resolved = vec![];

        // add the start node to visited list and paths.
        paths.push(vec![start]);

        // repeats until all edges are marked visited.
        while !paths.is_empty() {
            let path = paths.pop().unwrap();
            let last = *path.last().unwrap();
            let adjs = adjacents(last);
            for next in adjs {
                if path.contains(&next) {
                    continue; // prevent cycles
                }
                let mut new_path = path.clone();
                new_path.push(next);
                if next == end {
                    // reached to the end
                    resolved.push(new_path);
                } else {
                    // continue to traverse the graph
                    paths.push(new_path);
                }
            }
        }
        resolved
    }
}

impl<U: Copy + Eq> PathFinder<char> for LabeledGraph<char, U> {
    fn find_paths(&self, start: char, end: char) -> Vec<Vec<char>> {
        let adjacents = |v| self.get_node(&v).unwrap().adjacents();
        if let Some(_) = self.get_node(&start) {
            Self::find_all_paths(start, end, adjacents)
        } else {
            vec![]
        }
    }
}

impl<U: Copy + Eq> PathFinder<char> for LabeledDigraph<char, U> {
    fn find_paths(&self, start: char, end: char) -> Vec<Vec<char>> {
        let adjacents = |v| self.get_node(&v).unwrap().adjacents();
        if let Some(_) = self.get_node(&start) {
            Self::find_all_paths(start, end, adjacents)
        } else {
            vec![]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_paths_graph() {
        use P80::graph_converters::labeled;
        let g = labeled::from_string("[p-q/9, m-q/7, k, p-m/5]");
        assert_eq!(
            g.find_paths('p', 'q'),
            vec![vec!['p', 'q'], vec!['p', 'm', 'q']]
        );
        assert_eq!(g.find_paths('p', 'k'), Vec::<Vec<char>>::new());
    }

    #[test]
    fn test_find_paths_digraph() {
        use P80::digraph_converters::labeled;
        let g = labeled::from_string("[p>q/9, m>q/7, k, p>m/5]");
        assert_eq!(
            g.find_paths('p', 'q'),
            vec![vec!['p', 'q'], vec!['p', 'm', 'q']]
        );
        assert_eq!(g.find_paths('p', 'k'), Vec::<Vec<char>>::new());
    }
}
