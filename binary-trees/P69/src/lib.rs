use bintree_strrepr::Tree;

pub fn to_dotstring(tree: &Tree) -> String {
    match tree {
        Tree::Node { value, left, right } => {
            format!("{}{}{}", value, to_dotstring(left), to_dotstring(right))
        }
        Tree::End => ".".to_string(),
    }
}

pub fn from_dotstring(dotstr: &str) -> Tree {
    // TODO: refactor this
    fn from_dotstring_rec(s: &str) -> (Tree, &str) {
        assert!(!s.is_empty(), "Unaccepted input!");

        let (c, rem) = s.split_at(1);
        let (_, v) = c.char_indices().next().unwrap();
        if v == '.' {
            (Tree::end(), rem)
        } else {
            let mut t = Tree::leaf(v);
            let (left, rem1) = from_dotstring_rec(rem);
            let (right, rem2) = from_dotstring_rec(rem1);
            t.replace_left(left);
            t.replace_right(right);
            (t, rem2)
        }
    }

    let (t, rem) = from_dotstring_rec(dotstr);
    assert!(rem.is_empty(), "Unaceppted input!");
    t
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_dotstring() {
        assert_eq!(to_dotstring(&Tree::from_string("")), ".");
        assert_eq!(to_dotstring(&Tree::from_string("a")), "a..");
        assert_eq!(to_dotstring(&Tree::from_string("a(b,)")), "ab...");
        assert_eq!(to_dotstring(&Tree::from_string("a(,b)")), "a.b..");
        assert_eq!(to_dotstring(&Tree::from_string("a(b,c)")), "ab..c..");
        assert_eq!(
            to_dotstring(&Tree::from_string("a(b(d,e),c(,f(g,)))")),
            "abd..e..c.fg..."
        );
    }

    #[test]
    fn test_from_dotstring() {
        assert_eq!(from_dotstring("."), Tree::from_string(""));
        assert_eq!(from_dotstring("a.."), Tree::from_string("a"));
        assert_eq!(from_dotstring("ab..."), Tree::from_string("a(b,)"));
        assert_eq!(from_dotstring("a.b.."), Tree::from_string("a(,b)"));
        assert_eq!(from_dotstring("ab..c.."), Tree::from_string("a(b,c)"));
        assert_eq!(
            from_dotstring("abd..e..c.fg..."),
            Tree::from_string("a(b(d,e),c(,f(g,)))")
        );
    }
}
