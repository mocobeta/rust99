use P67::Tree;

/// Construct preorder sequence from a Tree
pub fn preorder(tree: &Tree) -> Vec<char> {
    let mut res = vec![];
    preorder_rec(tree, &mut res);
    res
}

fn preorder_rec(tree: &Tree, acc: &mut Vec<char>) {
    match tree {
        Tree::Node { value, left, right } => {
            acc.push(*value);
            preorder_rec(left, acc);
            preorder_rec(right, acc);
        }
        Tree::End => (),
    }
}

/// Construct Tree from a preorder sequence
pub fn from_preorder(seq: &Vec<char>) -> Vec<Tree> {
    if seq.len() == 0 {
        vec![]
    } else {
        let mut rem = seq.clone();
        let trees = vec![Tree::leaf(rem.remove(0))];
        let res = from_preorder_rec(&trees, &mut rem);
        res
    }
}

fn from_preorder_rec(trees: &Vec<Tree>, rem: &mut Vec<char>) -> Vec<Tree> {
    if rem.is_empty() {
        trees.clone()
    } else {
        let v = rem.remove(0);
        let new_trees = trees.iter().fold(vec![], |mut acc, tree| {
            let left = tree.get_left().unwrap();
            let right = tree.get_right().unwrap();
            match (left, right) {
                (Tree::End, Tree::Node { .. }) => {
                    // add new leaf to right tree
                    insert_leaf(right, v).into_iter().for_each(|subtree| {
                        let mut t = tree.clone();
                        t.replace_right(subtree);
                        acc.push(t);
                    });
                }
                _ => {
                    // add new leaf to left and right tree
                    insert_leaf(left, v).into_iter().for_each(|subtree| {
                        let mut t = tree.clone();
                        t.replace_left(subtree);
                        acc.push(t);
                    });
                    insert_leaf(right, v).into_iter().for_each(|subtree| {
                        let mut t = tree.clone();
                        t.replace_right(subtree);
                        acc.push(t);
                    });
                }
            }
            acc
        });
        from_preorder_rec(&new_trees, rem)
    }
}

fn insert_leaf(tree: &Tree, v: char) -> Vec<Tree> {
    if let Tree::Node {
        value: _,
        left,
        right,
    } = tree
    {
        let mut res: Vec<Tree> = vec![];
        if let Tree::End = left.as_ref() {
            let mut t = tree.clone();
            t.replace_left(Tree::leaf(v));
            res.push(t);
        } else {
            insert_leaf(left, v).into_iter().for_each(|x| {
                let mut t = tree.clone();
                t.replace_left(x);
                res.push(t);
            });
        }
        if let Tree::End = right.as_ref() {
            let mut t = tree.clone();
            t.replace_right(Tree::leaf(v));
            res.push(t);
        } else {
            insert_leaf(right, v).into_iter().for_each(|x| {
                let mut t = tree.clone();
                t.replace_right(x);
                res.push(t);
            });
        }
        res
    } else {
        vec![Tree::leaf(v)]
    }
}

/// Construct an inorder sequence from a Tree
pub fn inorder(tree: &Tree) -> Vec<char> {
    let mut res = vec![];
    inorder_rec(tree, &mut res);
    res
}

fn inorder_rec(tree: &Tree, acc: &mut Vec<char>) {
    match tree {
        Tree::Node { value, left, right } => {
            inorder_rec(left, acc);
            acc.push(*value);
            inorder_rec(right, acc);
        }
        Tree::End => (),
    }
}

/// Construct Tree from a inorder sequence
pub fn from_inorder(seq: &Vec<char>) -> Vec<Tree> {
    if seq.len() == 0 {
        vec![]
    } else {
        let mut rem = seq.clone();
        let mut trees = vec![Tree::leaf(rem.remove(0))];
        let res = from_inorder_rec(&mut trees, &mut rem);
        res
    }
}

fn from_inorder_rec(trees: &mut Vec<Tree>, rem: &mut Vec<char>) -> Vec<Tree> {
    if rem.is_empty() {
        trees.clone()
    } else {
        let v = rem.remove(0);
        let mut new_trees = trees.iter_mut().fold(vec![], |mut acc, tree| {
            // add existing tree to a new tree as left child
            let t = Tree::node(v, tree.clone(), Tree::End);
            acc.push(t);
            // add leaf node to right tree
            let mut t = tree.clone();
            insert_rightmost_leaf(&mut t, v);
            acc.push(t);
            if let Tree::Node { .. } = tree.get_right().unwrap() {
                let mut t = tree.clone();
                update_rightmost_tree(&mut t, v);
                acc.push(t);
            }
            acc
        });
        from_inorder_rec(&mut new_trees, rem)
    }
}

fn update_rightmost_tree(tree: &mut Tree, v: char) {
    if let Tree::Node {
        value: _,
        left: _,
        right,
    } = tree
    {
        if let Tree::Node {
            value: _,
            left: _,
            right: may_end,
        } = right.as_ref()
        {
            if let Tree::End = may_end.as_ref() {
                // swap
                let mut t = Tree::leaf(v);
                let new_left = right.as_ref().clone();
                t.replace_left(new_left);
                tree.replace_right(t);
            } else {
                update_rightmost_tree(right.as_mut(), v);
            }
        }
    }
}

fn insert_rightmost_leaf(tree: &mut Tree, v: char) {
    if let Tree::Node {
        value: _,
        left: _,
        right,
    } = tree
    {
        if let Tree::End = right.as_ref() {
            tree.replace_right(Tree::leaf(v));
        } else {
            insert_rightmost_leaf(right.as_mut(), v);
        }
    }
}

pub fn pre_in_tree(preorder: &Vec<char>, inorder: &Vec<char>) -> Vec<Tree> {
    let preorder_trees: Vec<String> = from_preorder(preorder)
        .into_iter()
        .map(|t| t.to_string())
        .collect();
    let inorder_trees: Vec<String> = from_inorder(inorder)
        .into_iter()
        .map(|t| t.to_string())
        .collect();

    let res = preorder_trees
        .into_iter()
        .filter(|tree| inorder_trees.contains(tree))
        .map(|s| Tree::from_string(&s))
        .collect();
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preorder() {
        assert_eq!(preorder(&Tree::from_string("")), vec![]);
        assert_eq!(preorder(&Tree::from_string("a")), vec!['a']);
        assert_eq!(preorder(&Tree::from_string("a(b,)")), vec!['a', 'b']);
        assert_eq!(preorder(&Tree::from_string("a(,c)")), vec!['a', 'c']);
        assert_eq!(preorder(&Tree::from_string("a(b,c)")), vec!['a', 'b', 'c']);
        assert_eq!(
            preorder(&Tree::from_string("a(b(d,e),c(,f(g,)))")),
            vec!['a', 'b', 'd', 'e', 'c', 'f', 'g']
        );
    }

    #[test]
    fn test_inorder() {
        assert_eq!(inorder(&Tree::from_string("")), vec![]);
        assert_eq!(inorder(&Tree::from_string("a")), vec!['a']);
        assert_eq!(inorder(&Tree::from_string("a(b,)")), vec!['b', 'a']);
        assert_eq!(inorder(&Tree::from_string("a(,c)")), vec!['a', 'c']);
        assert_eq!(inorder(&Tree::from_string("a(b,c)")), vec!['b', 'a', 'c']);
        assert_eq!(
            inorder(&Tree::from_string("a(b(d,e),c(,f(g,)))")),
            vec!['d', 'b', 'e', 'a', 'c', 'g', 'f']
        );
    }

    #[test]
    fn test_from_preorder() {
        assert_eq!(from_preorder(&Vec::<char>::new()), vec![]);
        assert_eq!(from_preorder(&vec!['a']), vec![Tree::from_string("a")]);
        assert_eq!(
            from_preorder(&vec!['a', 'b']),
            vec![Tree::from_string("a(b,)"), Tree::from_string("a(,b)")]
        );

        let trees = from_preorder(&vec!['a', 'b', 'c']);
        assert_eq!(trees.len(), 5);
        for tree in trees {
            assert_eq!(preorder(&tree), vec!['a', 'b', 'c']);
        }

        let trees = from_preorder(&vec!['a', 'b', 'd', 'e', 'c', 'f', 'g']);
        assert!(trees.contains(&Tree::from_string("a(b(d,e),c(,f(g,)))")));
    }

    #[test]
    fn test_from_inorder() {
        assert_eq!(from_inorder(&Vec::<char>::new()), vec![]);
        assert_eq!(from_inorder(&vec!['a']), vec![Tree::from_string("a")]);
        assert_eq!(
            from_inorder(&vec!['a', 'b']),
            vec![Tree::from_string("b(a,)"), Tree::from_string("a(,b)")]
        );

        let trees = from_inorder(&vec!['a', 'b', 'c']);
        assert_eq!(trees.len(), 5);
        for tree in trees {
            assert_eq!(inorder(&tree), vec!['a', 'b', 'c']);
        }

        let trees = from_inorder(&vec!['d', 'b', 'e', 'a', 'c', 'g', 'f']);
        assert!(trees.contains(&Tree::from_string("a(b(d,e),c(,f(g,)))")));
    }

    #[test]
    fn test_pre_in_tree() {
        let trees = pre_in_tree(
            &vec!['a', 'b', 'd', 'e', 'c', 'f', 'g'],
            &vec!['d', 'b', 'e', 'a', 'c', 'g', 'f'],
        );
        assert_eq!(trees.len(), 1);
        assert!(trees.contains(&Tree::from_string("a(b(d,e),c(,f(g,)))")));
    }

    #[test]
    fn test_add_leaf() {
        let tree = Tree::end();
        assert_eq!(insert_leaf(&tree, 'a'), vec![Tree::from_string("a")]);

        let tree = Tree::from_string("a");
        assert_eq!(
            insert_leaf(&tree, 'b'),
            vec![Tree::from_string("a(b,)"), Tree::from_string("a(,b)")]
        );

        let tree = Tree::from_string("a(b,)");
        assert_eq!(
            insert_leaf(&tree, 'c'),
            vec![
                Tree::from_string("a(b(c,),)"),
                Tree::from_string("a(b(,c))"),
                Tree::from_string("a(b,c)")
            ]
        );
    }
}
