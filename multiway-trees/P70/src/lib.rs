use mtree::MTree;

/// Parses string and construct MTree.
/// BNF syntax:
/// <Value>    ::= Character
/// <Node>     ::= <Value> <Children>
/// <Children> ::= '^' | <Node> <Children>
pub fn str_to_tree(s: &str) -> MTree {
    fn parse_node(s: &str) -> (MTree, &str) {
        let (c, last) = next_token(s);
        let (children, rem) = parse_children(last);
        (MTree::node(c, children), rem)
    }

    fn parse_children(s: &str) -> (Vec<MTree>, &str) {
        let (c, last) = next_token(s);
        match c {
            '^' => (vec![], last),
            _ => {
                let mut res = vec![];
                let (node, rem) = parse_node(s);
                res.push(node);
                let (children, rem) = parse_children(rem);
                res.extend_from_slice(&children);
                (res, rem)
            }
        }
    }

    fn next_token(s: &str) -> (char, &str) {
        let (first, last) = s.split_at(1);
        let c = first.chars().nth(0).unwrap();
        (c, last)
    }

    parse_node(s).0
}

/// Converts Mtree to string representation
pub fn tree_to_str(tree: &MTree) -> String {
    fn tree_to_str_inner(tree: &MTree, acc: &mut String) {
        acc.push(tree.get_value());
        for child in tree.get_children() {
            tree_to_str_inner(child, acc);
        }
        acc.push('^'); // add backtrack mark
    }

    let mut res = String::new();
    tree_to_str_inner(tree, &mut res);
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_str_to_tree() {
        assert_eq!(str_to_tree("a^"), MTree::leaf('a'));
        assert_eq!(
            str_to_tree("ab^^"),
            MTree::node('a', vec![MTree::leaf('b')])
        );
        assert_eq!(
            str_to_tree("ab^c^d^^"),
            MTree::node(
                'a',
                vec![MTree::leaf('b'), MTree::leaf('c'), MTree::leaf('d')]
            )
        );
        assert_eq!(
            str_to_tree("abc^^^"),
            MTree::node('a', vec![MTree::node('b', vec![MTree::leaf('c')])])
        );
        assert_eq!(
            str_to_tree("afg^^c^bd^e^^^"),
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

    #[test]
    fn test_tree_to_str() {
        assert_eq!(tree_to_str(&MTree::leaf('a')), "a^");
        assert_eq!(
            tree_to_str(&MTree::node('a', vec![MTree::leaf('b')])),
            "ab^^"
        );
        assert_eq!(
            tree_to_str(&MTree::node(
                'a',
                vec![MTree::leaf('b'), MTree::leaf('c'), MTree::leaf('d')]
            )),
            "ab^c^d^^"
        );
        assert_eq!(
            tree_to_str(&MTree::node(
                'a',
                vec![MTree::node('b', vec![MTree::leaf('c')])]
            )),
            "abc^^^"
        );
        assert_eq!(
            tree_to_str(&MTree::node(
                'a',
                vec![
                    MTree::node('f', vec![MTree::leaf('g')]),
                    MTree::leaf('c'),
                    MTree::node('b', vec![MTree::leaf('d'), MTree::leaf('e')])
                ]
            )),
            "afg^^c^bd^e^^^"
        );
    }
}
