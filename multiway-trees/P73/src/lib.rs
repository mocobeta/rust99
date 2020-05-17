use mtree::MTree;

pub fn lispy_tree(tree: &MTree) -> String {
    if tree.get_children().is_empty() {
        let mut s = String::new();
        s.push(tree.get_value());
        s
    } else {
        let mut s = String::new();
        s.push('(');
        s.push(tree.get_value());
        for child in tree.get_children() {
            s.push(' ');
            s.push_str(&lispy_tree(child));
        }
        s.push(')');
        s
    }
}

/// Parse lispy string and construct MTree
/// BNF syntax:
/// <Value>    ::= Character
/// <Node>     ::= <Value> | '(' <Children> ')'
/// <Children> ::= <Node> | <Node> ' ' <Children>
pub fn lispy_str_to_tree(s: &str) -> MTree {
    parse_node(s).0
}

fn parse_node(s: &str) -> (MTree, &str) {
    let (c, last) = next_token(s);
    match c {
        '(' => {
            let (v, last) = next_token(last);
            let (c, last) = next_token(last);
            if c != ' ' {
                panic!("Invalid lispy string");
            }
            let (children, last) = parse_children(last);
            let (c, last) = next_token(last);
            if c != ')' {
                panic!("Invalid lispy string");
            }
            (MTree::node(v, children), last)
        }
        ' ' => parse_node(last),
        _ => (MTree::leaf(c), last),
    }
}

fn parse_children(s: &str) -> (Vec<MTree>, &str) {
    let mut res = vec![];
    let (node, last) = parse_node(s);
    res.push(node);

    let (c, _) = next_token(last);
    match c {
        ')' => (res, last),
        _ => {
            let (nodes, last) = parse_children(last);
            for node in nodes {
                res.push(node);
            }
            (res, last)
        }
    }
}

fn next_token(s: &str) -> (char, &str) {
    let (first, last) = s.split_at(1);
    let c = first.chars().nth(0).unwrap();
    (c, last)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lispy_tree() {
        assert_eq!(lispy_tree(&MTree::leaf('a')), "a");
        assert_eq!(
            lispy_tree(&MTree::node('a', vec![MTree::leaf('b')])),
            "(a b)"
        );
        assert_eq!(
            lispy_tree(&MTree::node(
                'a',
                vec![MTree::node('b', vec![MTree::leaf('c')])]
            )),
            "(a (b c))"
        );
        assert_eq!(
            lispy_tree(&MTree::node('b', vec![MTree::leaf('d'), MTree::leaf('e')])),
            "(b d e)"
        );
        assert_eq!(
            lispy_tree(&MTree::node(
                'a',
                vec![
                    MTree::node('f', vec![MTree::leaf('g')]),
                    MTree::leaf('c'),
                    MTree::node('b', vec![MTree::leaf('d'), MTree::leaf('e')])
                ]
            )),
            "(a (f g) c (b d e))"
        );
    }

    #[test]
    fn test_lispy_str_to_tree() {
        assert_eq!(lispy_str_to_tree("a"), MTree::leaf('a'));
        assert_eq!(
            lispy_str_to_tree("(a b)"),
            MTree::node('a', vec![MTree::leaf('b')])
        );
        assert_eq!(
            lispy_str_to_tree("(a (b c))"),
            MTree::node('a', vec![MTree::node('b', vec![MTree::leaf('c')])]),
        );
        assert_eq!(
            lispy_str_to_tree("(b d e)"),
            MTree::node('b', vec![MTree::leaf('d'), MTree::leaf('e')])
        );
        assert_eq!(
            lispy_str_to_tree("(a (f g) c (b d e))"),
            MTree::node(
                'a',
                vec![
                    MTree::node('f', vec![MTree::leaf('g')]),
                    MTree::leaf('c'),
                    MTree::node('b', vec![MTree::leaf('d'), MTree::leaf('e')])
                ]
            )
        );
    }
}
