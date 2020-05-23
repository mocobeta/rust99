pub mod unlabeled {
    use graph::Graph;
    use std::collections::HashSet;

    pub fn from_term_form(nodes: &Vec<char>, edges: &Vec<(char, char)>) -> Graph<char> {
        let mut g = Graph::new();
        for node in nodes {
            g.add_node(*node);
        }
        for (n1, n2) in edges {
            g.add_edge(*n1, *n2);
        }
        g
    }

    pub fn to_term_form(g: &Graph<char>) -> (Vec<char>, Vec<(char, char)>) {
        let all_nodes = g.get_nodes();
        let nodes: Vec<char> = all_nodes.iter().map(|node| *node.get_value()).collect();
        let edges: HashSet<(char, char)> = all_nodes
            .iter()
            .flat_map(|node| {
                let pairs: Vec<(char, char)> = node
                    .adjacents()
                    .iter()
                    .map(|adj| {
                        if node.get_value() < adj {
                            (*node.get_value(), *adj)
                        } else {
                            (*adj, *node.get_value())
                        }
                    })
                    .collect();
                pairs
            })
            .collect();
        (nodes, edges.into_iter().collect())
    }

    pub fn from_adjacent_form(adj_list: &Vec<(char, &Vec<char>)>) -> Graph<char> {
        let nodes: Vec<&char> = adj_list.iter().map(|(node, _)| node).collect();
        let edges: HashSet<(char, char)> = adj_list
            .iter()
            .flat_map(|(node, adjs)| {
                let pairs: Vec<(char, char)> = adjs
                    .iter()
                    .map(|adj| {
                        if node < adj {
                            (*node, *adj)
                        } else {
                            (*adj, *node)
                        }
                    })
                    .collect();
                pairs
            })
            .collect();

        let mut g = Graph::new();
        for node in nodes {
            g.add_node(*node);
        }
        for (n1, n2) in edges {
            g.add_edge(n1, n2);
        }
        g
    }

    pub fn to_adjacent_form(g: &Graph<char>) -> Vec<(char, Vec<char>)> {
        g.get_nodes()
            .into_iter()
            .map(|node| (*node.get_value(), node.adjacents()))
            .collect()
    }

    pub fn from_string(s: &str) -> Graph<char> {
        let slen = s.chars().count();
        if slen < 2 || s.chars().nth(0).unwrap() != '[' || s.chars().nth(slen - 1).unwrap() != ']' {
            panic!("Invalid string format: {}", s);
        }
        let s = &s[1..slen - 1]; // assume ascii characters only
        let pairs: Vec<String> = s.split(' ').map(|p| p.replace(",", "")).collect();

        // check format
        if !pairs
            .iter()
            .all(|pair| pair.len() == 1 || (pair.len() == 3 && pair.chars().nth(1).unwrap() == '-'))
        {
            panic!("Invalid string format: {}", s);
        }

        let nodes: HashSet<char> = pairs
            .iter()
            .flat_map(|p| {
                if p.len() == 1 {
                    vec![p.chars().nth(0).unwrap()]
                } else {
                    vec![p.chars().nth(0).unwrap(), p.chars().nth(2).unwrap()]
                }
            })
            .collect();
        let edges: HashSet<(char, char)> = pairs
            .iter()
            .filter(|p| p.len() == 3)
            .map(|p| {
                let v1 = p.chars().nth(0).unwrap();
                let v2 = p.chars().nth(2).unwrap();
                if v1 < v2 {
                    (v1, v2)
                } else {
                    (v2, v1)
                }
            })
            .collect();

        let mut g = Graph::new();
        for node in nodes {
            g.add_node(node);
        }
        for (n1, n2) in edges {
            g.add_edge(n1, n2);
        }
        g
    }
}

#[cfg(test)]
mod tests_unlabeled {
    use super::test_util::has_same_elements;
    use super::unlabeled::*;
    use graph::Graph;

    #[test]
    fn test_from_term_form() {
        let g = from_term_form(
            &vec!['b', 'c', 'd', 'f', 'g', 'h', 'k'],
            &vec![('b', 'c'), ('b', 'f'), ('c', 'f'), ('f', 'k'), ('g', 'h')],
        );
        check_graph(&g);
    }

    #[test]
    fn test_to_term_form() {
        let (nodes, edges) = to_term_form(&make_test_graph());
        assert!(has_same_elements(
            &nodes,
            &vec!['b', 'c', 'd', 'f', 'g', 'h', 'k']
        ));
        assert!(has_same_elements(
            &edges,
            &vec![('b', 'c'), ('b', 'f'), ('c', 'f'), ('f', 'k'), ('g', 'h')]
        ));
    }

    #[test]
    fn test_from_adjacent_form() {
        let g = from_adjacent_form(&vec![
            ('b', &vec!['c', 'f']),
            ('c', &vec!['b', 'f']),
            ('d', &vec![]),
            ('f', &vec!['b', 'c', 'k']),
            ('g', &vec!['h']),
            ('h', &vec!['g']),
            ('k', &vec!['f']),
        ]);
        check_graph(&g);
    }

    #[test]
    fn test_to_adjacent_form() {
        let mut adj_list = to_adjacent_form(&make_test_graph());
        adj_list.sort_by(|(v1, _), (v2, _)| v1.cmp(v2));
        assert_eq!(adj_list[0].0, 'b');
        assert!(has_same_elements(&adj_list[0].1, &vec!['c', 'f']));
        assert_eq!(adj_list[1].0, 'c');
        assert!(has_same_elements(&adj_list[1].1, &vec!['b', 'f']));
        assert_eq!(adj_list[2].0, 'd');
        assert!(has_same_elements(&adj_list[2].1, &vec![]));
        assert_eq!(adj_list[3].0, 'f');
        assert!(has_same_elements(&adj_list[3].1, &vec!['b', 'c', 'k']));
        assert_eq!(adj_list[4].0, 'g');
        assert!(has_same_elements(&adj_list[4].1, &vec!['h']));
        assert_eq!(adj_list[5].0, 'h');
        assert!(has_same_elements(&adj_list[5].1, &vec!['g']));
        assert_eq!(adj_list[6].0, 'k');
        assert!(has_same_elements(&adj_list[6].1, &vec!['f']));
    }

    #[test]
    fn test_from_string() {
        let g = from_string("[b-c, f-c, g-h, d, f-b, k-f, h-g]");
        check_graph(&g);
    }

    fn check_graph(g: &Graph<char>) {
        assert_eq!(g.size(), 7);
        assert!(has_same_elements(
            &g.get_node(&'b').unwrap().adjacents(),
            &vec!['c', 'f']
        ));
        assert!(has_same_elements(
            &g.get_node(&'c').unwrap().adjacents(),
            &vec!['b', 'f']
        ));
        assert!(has_same_elements(
            &g.get_node(&'d').unwrap().adjacents(),
            &vec![]
        ));
        assert!(has_same_elements(
            &g.get_node(&'f').unwrap().adjacents(),
            &vec!['b', 'c', 'k']
        ));
        assert!(has_same_elements(
            &g.get_node(&'g').unwrap().adjacents(),
            &vec!['h']
        ));
        assert!(has_same_elements(
            &g.get_node(&'h').unwrap().adjacents(),
            &vec!['g']
        ));
        assert!(has_same_elements(
            &g.get_node(&'k').unwrap().adjacents(),
            &vec!['f']
        ));
    }

    fn make_test_graph() -> Graph<char> {
        let mut g = Graph::new();
        g.add_node('b');
        g.add_node('c');
        g.add_node('d');
        g.add_node('f');
        g.add_node('g');
        g.add_node('h');
        g.add_node('k');
        g.add_edge('b', 'c');
        g.add_edge('b', 'f');
        g.add_edge('c', 'f');
        g.add_edge('f', 'k');
        g.add_edge('g', 'h');
        g
    }
}

pub mod labeled {
    use graph::LabeledGraph;
    use std::collections::HashSet;

    pub fn from_term_form(
        nodes: &Vec<char>,
        edges: &Vec<(char, char, i32)>,
    ) -> LabeledGraph<char, i32> {
        let mut g = LabeledGraph::new();
        for node in nodes {
            g.add_node(*node);
        }
        for (n1, n2, v) in edges {
            g.add_labeled_edge(*n1, *n2, *v);
        }
        g
    }

    pub fn to_term_form(g: &LabeledGraph<char, i32>) -> (Vec<char>, Vec<(char, char, i32)>) {
        let all_nodes = g.get_nodes();
        let nodes: Vec<char> = all_nodes.iter().map(|node| *node.get_value()).collect();
        let edges: HashSet<(char, char, i32)> = all_nodes
            .iter()
            .flat_map(|node| {
                let pairs: Vec<(char, char, i32)> = node
                    .adjacents_with_label()
                    .iter()
                    .map(|(adj, label)| {
                        if node.get_value() < adj {
                            (*node.get_value(), *adj, label.unwrap())
                        } else {
                            (*adj, *node.get_value(), label.unwrap())
                        }
                    })
                    .collect();
                pairs
            })
            .collect();
        (nodes, edges.into_iter().collect())
    }

    pub fn from_adjacent_form(
        adj_list: &Vec<(char, &Vec<(char, i32)>)>,
    ) -> LabeledGraph<char, i32> {
        let nodes: Vec<&char> = adj_list.iter().map(|(node, _)| node).collect();
        let edges: HashSet<(char, char, i32)> = adj_list
            .iter()
            .flat_map(|(node, adjs)| {
                let pairs: Vec<(char, char, i32)> = adjs
                    .iter()
                    .map(|(adj, label)| {
                        if node < adj {
                            (*node, *adj, *label)
                        } else {
                            (*adj, *node, *label)
                        }
                    })
                    .collect();
                pairs
            })
            .collect();

        let mut g = LabeledGraph::new();
        for node in nodes {
            g.add_node(*node);
        }
        for (n1, n2, l) in edges {
            g.add_labeled_edge(n1, n2, l);
        }
        g
    }

    pub fn to_adjacent_form(g: &LabeledGraph<char, i32>) -> Vec<(char, Vec<(char, i32)>)> {
        g.get_nodes()
            .into_iter()
            .map(|node| {
                (
                    *node.get_value(),
                    node.adjacents_with_label()
                        .into_iter()
                        .map(|(adj, label)| (adj, label.unwrap()))
                        .collect(),
                )
            })
            .collect()
    }

    pub fn from_string(s: &str) -> LabeledGraph<char, i32> {
        let slen = s.chars().count();
        if slen < 2 || s.chars().nth(0).unwrap() != '[' || s.chars().nth(slen - 1).unwrap() != ']' {
            panic!("Invalid string format: {}", s);
        }
        let s = &s[1..slen - 1]; // assume ascii characters only
        let pairs: Vec<String> = s.split(' ').map(|p| p.replace(",", "")).collect();

        // check format
        if !pairs.iter().all(|pair| {
            pair.len() == 1
                || (pair.len() >= 5
                    && pair.chars().nth(1).unwrap() == '-'
                    && pair.chars().nth(3).unwrap() == '/')
        }) {
            panic!("Invalid string format: {}", s);
        }

        let nodes: HashSet<char> = pairs
            .iter()
            .flat_map(|p| {
                if p.len() == 1 {
                    vec![p.chars().nth(0).unwrap()]
                } else {
                    vec![p.chars().nth(0).unwrap(), p.chars().nth(2).unwrap()]
                }
            })
            .collect();
        let edges: HashSet<(char, char, i32)> = pairs
            .iter()
            .filter(|p| p.len() >= 5)
            .map(|p| {
                let v1 = p.chars().nth(0).unwrap();
                let v2 = p.chars().nth(2).unwrap();
                let (_, label) = p.split_at(4);
                let label: i32 = label.parse().unwrap();
                if v1 < v2 {
                    (v1, v2, label)
                } else {
                    (v2, v1, label)
                }
            })
            .collect();

        let mut g = LabeledGraph::new();
        for node in nodes {
            g.add_node(node);
        }
        for (n1, n2, l) in edges {
            g.add_labeled_edge(n1, n2, l);
        }
        g
    }
}

#[cfg(test)]
mod tests_labeled {
    use super::labeled::*;
    use super::test_util::has_same_elements;
    use graph::LabeledGraph;

    #[test]
    fn test_from_term_form() {
        let g = from_term_form(
            &vec!['k', 'm', 'p', 'q'],
            &vec![('m', 'p', 5), ('m', 'q', 7), ('p', 'q', 9)],
        );
        check_graph(&g);
    }

    #[test]
    fn test_to_term_form() {
        let (nodes, edges) = to_term_form(&make_test_graph());
        assert!(has_same_elements(&nodes, &vec!['k', 'm', 'p', 'q']));
        assert!(has_same_elements(
            &edges,
            &vec![('m', 'p', 5), ('m', 'q', 7), ('p', 'q', 9)]
        ));
    }

    #[test]
    fn test_from_adjacent_form() {
        let g = from_adjacent_form(&vec![
            ('k', &vec![]),
            ('m', &vec![('p', 5), ('q', 7)]),
            ('p', &vec![('m', 5), ('q', 9)]),
            ('q', &vec![('m', 7), ('p', 9)]),
        ]);
        check_graph(&g);
    }

    #[test]
    fn test_to_adjacent_form() {
        let mut adj_list = to_adjacent_form(&make_test_graph());
        adj_list.sort_by(|(v1, _), (v2, _)| v1.cmp(v2));
        assert_eq!(adj_list[0].0, 'k');
        assert!(has_same_elements(&adj_list[0].1, &vec![]));
        assert_eq!(adj_list[1].0, 'm');
        assert!(has_same_elements(&adj_list[1].1, &vec![('p', 5), ('q', 7)]));
        assert_eq!(adj_list[2].0, 'p');
        assert!(has_same_elements(&adj_list[2].1, &vec![('m', 5), ('q', 9)]));
        assert_eq!(adj_list[3].0, 'q');
        assert!(has_same_elements(&adj_list[3].1, &vec![('m', 7), ('p', 9)]));
    }

    #[test]
    fn test_from_string() {
        let g = from_string("[k, m-p/5, m-q/7, p-q/9]");
        check_graph(&g);
    }

    fn check_graph(g: &LabeledGraph<char, i32>) {
        assert_eq!(g.size(), 4);
        assert!(has_same_elements(
            &g.get_node(&'k').unwrap().adjacents(),
            &vec![]
        ));
        assert!(has_same_elements(
            &g.get_node(&'m').unwrap().adjacents_with_label(),
            &vec![('p', Some(5)), ('q', Some(7))]
        ));
        assert!(has_same_elements(
            &g.get_node(&'p').unwrap().adjacents_with_label(),
            &vec![('m', Some(5)), ('q', Some(9))]
        ));
        assert!(has_same_elements(
            &g.get_node(&'q').unwrap().adjacents_with_label(),
            &vec![('m', Some(7)), ('p', Some(9))]
        ));
    }

    fn make_test_graph() -> LabeledGraph<char, i32> {
        let mut g = LabeledGraph::new();
        g.add_node('k');
        g.add_node('m');
        g.add_node('p');
        g.add_node('q');
        g.add_labeled_edge('m', 'p', 5);
        g.add_labeled_edge('m', 'q', 7);
        g.add_labeled_edge('p', 'q', 9);
        g
    }
}

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
