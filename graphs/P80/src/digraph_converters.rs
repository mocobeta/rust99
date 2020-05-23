pub mod unlabeled {
    use graph::Digraph;
    use std::collections::HashSet;

    pub fn from_term_form(nodes: &Vec<char>, edges: &Vec<(char, char)>) -> Digraph<char> {
        let mut g = Digraph::new();
        for node in nodes {
            g.add_node(*node);
        }
        for (n1, n2) in edges {
            g.add_arc(*n1, *n2);
        }
        g
    }

    pub fn to_term_form(g: &Digraph<char>) -> (Vec<char>, Vec<(char, char)>) {
        let all_nodes = g.get_nodes();
        let nodes: Vec<char> = all_nodes.iter().map(|node| *node.get_value()).collect();
        let edges: HashSet<(char, char)> = all_nodes
            .iter()
            .flat_map(|node| {
                let pairs: Vec<(char, char)> = node
                    .adjacents()
                    .iter()
                    .map(|adj| (*node.get_value(), *adj))
                    .collect();
                pairs
            })
            .collect();
        (nodes, edges.into_iter().collect())
    }

    pub fn from_adjacent_form(adj_list: &Vec<(char, &Vec<char>)>) -> Digraph<char> {
        let nodes: Vec<&char> = adj_list.iter().map(|(node, _)| node).collect();
        let edges: HashSet<(char, char)> = adj_list
            .iter()
            .flat_map(|(node, adjs)| {
                let pairs: Vec<(char, char)> = adjs.iter().map(|adj| (*node, *adj)).collect();
                pairs
            })
            .collect();

        let mut g = Digraph::new();
        for node in nodes {
            g.add_node(*node);
        }
        for (n1, n2) in edges {
            g.add_arc(n1, n2);
        }
        g
    }

    pub fn to_adjacent_form(g: &Digraph<char>) -> Vec<(char, Vec<char>)> {
        g.get_nodes()
            .into_iter()
            .map(|node| (*node.get_value(), node.adjacents()))
            .collect()
    }

    pub fn from_string(s: &str) -> Digraph<char> {
        let slen = s.chars().count();
        if slen < 2 || s.chars().nth(0).unwrap() != '[' || s.chars().nth(slen - 1).unwrap() != ']' {
            panic!("Invalid string format: {}", s);
        }
        let s = &s[1..slen - 1]; // assume ascii characters only
        let pairs: Vec<String> = s.split(' ').map(|p| p.replace(",", "")).collect();

        // check format
        if !pairs
            .iter()
            .all(|pair| pair.len() == 1 || (pair.len() == 3 && pair.chars().nth(1).unwrap() == '>'))
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
                (v1, v2)
            })
            .collect();

        let mut g = Digraph::new();
        for node in nodes {
            g.add_node(node);
        }
        for (n1, n2) in edges {
            g.add_arc(n1, n2);
        }
        g
    }
}

#[cfg(test)]
mod test_unlabeled {
    use super::test_util::has_same_elements;
    use super::unlabeled::*;
    use graph::Digraph;

    #[test]
    fn test_from_term_form() {
        let g = from_term_form(
            &vec!['r', 's', 't', 'u', 'v'],
            &vec![('s', 'r'), ('s', 'u'), ('u', 'r'), ('u', 's'), ('v', 'u')],
        );
        check_graph(&g);
    }

    #[test]
    fn test_to_term_form() {
        let (nodes, edges) = to_term_form(&make_test_graph());
        assert!(has_same_elements(&nodes, &vec!['r', 's', 't', 'u', 'v']));
        assert!(has_same_elements(
            &edges,
            &vec![('s', 'r'), ('s', 'u'), ('u', 'r'), ('u', 's'), ('v', 'u')]
        ));
    }

    #[test]
    fn test_from_adjacent_form() {
        let g = from_adjacent_form(&vec![
            ('r', &vec![]),
            ('s', &vec!['r', 'u']),
            ('t', &vec![]),
            ('u', &vec!['r', 's']),
            ('v', &vec!['u']),
        ]);
        check_graph(&g);
    }

    #[test]
    fn test_to_adjacent_form() {
        let mut adj_list = to_adjacent_form(&make_test_graph());
        adj_list.sort_by(|(v1, _), (v2, _)| v1.cmp(v2));
        assert_eq!(adj_list[0].0, 'r');
        assert!(has_same_elements(&adj_list[0].1, &vec![]));
        assert_eq!(adj_list[1].0, 's');
        assert!(has_same_elements(&adj_list[1].1, &vec!['r', 'u']));
        assert_eq!(adj_list[2].0, 't');
        assert!(has_same_elements(&adj_list[2].1, &vec![]));
        assert_eq!(adj_list[3].0, 'u');
        assert!(has_same_elements(&adj_list[3].1, &vec!['r', 's']));
        assert_eq!(adj_list[4].0, 'v');
        assert!(has_same_elements(&adj_list[4].1, &vec!['u']));
    }

    #[test]
    fn test_from_string() {
        let g = from_string("[s>r, t, u>r, s>u, u>s, v>u]");
        check_graph(&g);
    }

    fn check_graph(g: &Digraph<char>) {
        assert_eq!(g.size(), 5);
        assert!(has_same_elements(
            &g.get_node(&'r').unwrap().adjacents(),
            &vec![]
        ));
        assert!(has_same_elements(
            &g.get_node(&'s').unwrap().adjacents(),
            &vec!['r', 'u']
        ));
        assert!(has_same_elements(
            &g.get_node(&'t').unwrap().adjacents(),
            &vec![]
        ));
        assert!(has_same_elements(
            &g.get_node(&'u').unwrap().adjacents(),
            &vec!['r', 's']
        ));
        assert!(has_same_elements(
            &g.get_node(&'v').unwrap().adjacents(),
            &vec!['u']
        ));
    }

    fn make_test_graph() -> Digraph<char> {
        let mut g = Digraph::new();
        g.add_node('r');
        g.add_node('s');
        g.add_node('t');
        g.add_node('u');
        g.add_node('v');
        g.add_arc('s', 'r');
        g.add_arc('s', 'u');
        g.add_arc('u', 'r');
        g.add_arc('u', 's');
        g.add_arc('v', 'u');
        g
    }
}

pub mod labeled {
    use graph::LabeledDigraph;
    use std::collections::HashSet;

    pub fn from_term_form(
        nodes: &Vec<char>,
        edges: &Vec<(char, char, i32)>,
    ) -> LabeledDigraph<char, i32> {
        let mut g = LabeledDigraph::new();
        for node in nodes {
            g.add_node(*node);
        }
        for (n1, n2, v) in edges {
            g.add_labeled_arc(*n1, *n2, *v);
        }
        g
    }

    pub fn to_term_form(g: &LabeledDigraph<char, i32>) -> (Vec<char>, Vec<(char, char, i32)>) {
        let all_nodes = g.get_nodes();
        let nodes: Vec<char> = all_nodes.iter().map(|node| *node.get_value()).collect();
        let edges: HashSet<(char, char, i32)> = all_nodes
            .iter()
            .flat_map(|node| {
                let pairs: Vec<(char, char, i32)> = node
                    .adjacents_with_label()
                    .iter()
                    .map(|(adj, label)| (*node.get_value(), *adj, label.unwrap()))
                    .collect();
                pairs
            })
            .collect();
        (nodes, edges.into_iter().collect())
    }

    pub fn from_adjacent_form(
        adj_list: &Vec<(char, &Vec<(char, i32)>)>,
    ) -> LabeledDigraph<char, i32> {
        let nodes: Vec<&char> = adj_list.iter().map(|(node, _)| node).collect();
        let edges: HashSet<(char, char, i32)> = adj_list
            .iter()
            .flat_map(|(node, adjs)| {
                let pairs: Vec<(char, char, i32)> = adjs
                    .iter()
                    .map(|(adj, label)| (*node, *adj, *label))
                    .collect();
                pairs
            })
            .collect();

        let mut g = LabeledDigraph::new();
        for node in nodes {
            g.add_node(*node);
        }
        for (n1, n2, l) in edges {
            g.add_labeled_arc(n1, n2, l);
        }
        g
    }

    pub fn to_adjacent_form(g: &LabeledDigraph<char, i32>) -> Vec<(char, Vec<(char, i32)>)> {
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

    pub fn from_string(s: &str) -> LabeledDigraph<char, i32> {
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
                    && pair.chars().nth(1).unwrap() == '>'
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
                (v1, v2, label)
            })
            .collect();

        let mut g = LabeledDigraph::new();
        for node in nodes {
            g.add_node(node);
        }
        for (n1, n2, l) in edges {
            g.add_labeled_arc(n1, n2, l);
        }
        g
    }
}

#[cfg(test)]
mod test_labeled {
    use super::labeled::*;
    use super::test_util::has_same_elements;
    use graph::LabeledDigraph;

    #[test]
    fn test_from_term_form() {
        let g = from_term_form(
            &vec!['k', 'm', 'p', 'q'],
            &vec![('p', 'm', 5), ('m', 'q', 7), ('p', 'q', 9)],
        );
        check_graph(&g);
    }

    #[test]
    fn test_to_term_form() {
        let (nodes, edges) = to_term_form(&make_test_graph());
        assert!(has_same_elements(&nodes, &vec!['k', 'm', 'p', 'q']));
        assert!(has_same_elements(
            &edges,
            &vec![('p', 'm', 5), ('m', 'q', 7), ('p', 'q', 9)]
        ));
    }

    #[test]
    fn test_from_adjacent_form() {
        let g = from_adjacent_form(&vec![
            ('k', &vec![]),
            ('m', &vec![('q', 7)]),
            ('p', &vec![('m', 5), ('q', 9)]),
            ('q', &vec![]),
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
        assert!(has_same_elements(&adj_list[1].1, &vec![('q', 7)]));
        assert_eq!(adj_list[2].0, 'p');
        assert!(has_same_elements(&adj_list[2].1, &vec![('m', 5), ('q', 9)]));
        assert_eq!(adj_list[3].0, 'q');
        assert!(has_same_elements(&adj_list[3].1, &vec![]));
    }

    #[test]
    fn test_from_string() {
        let g = from_string("[k, p>m/5, m>q/7, p>q/9]");
        check_graph(&g);
    }

    fn check_graph(g: &LabeledDigraph<char, i32>) {
        assert_eq!(g.size(), 4);
        assert!(has_same_elements(
            &g.get_node(&'k').unwrap().adjacents(),
            &vec![]
        ));
        assert!(has_same_elements(
            &g.get_node(&'m').unwrap().adjacents_with_label(),
            &vec![('q', Some(7))]
        ));
        assert!(has_same_elements(
            &g.get_node(&'p').unwrap().adjacents_with_label(),
            &vec![('m', Some(5)), ('q', Some(9))]
        ));
        assert!(has_same_elements(
            &g.get_node(&'q').unwrap().adjacents_with_label(),
            &vec![]
        ));
    }

    fn make_test_graph() -> LabeledDigraph<char, i32> {
        let mut g = LabeledDigraph::new();
        g.add_node('k');
        g.add_node('m');
        g.add_node('p');
        g.add_node('q');
        g.add_labeled_arc('m', 'q', 7);
        g.add_labeled_arc('p', 'm', 5);
        g.add_labeled_arc('p', 'q', 9);
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
