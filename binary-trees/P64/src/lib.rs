use bintree::Tree;
use bintree_pos::PositionedTree;
use std::fmt;

pub fn layout_bintree<T: Copy + fmt::Display>(tree: &Tree<T>) -> PositionedTree<T> {
    fn layout_rec<T: Copy + fmt::Display>(
        node: &Tree<T>,
        ord: u32,
        depth: u32,
    ) -> PositionedTree<T> {
        match node {
            Tree::Node { value, left, right } => {
                // creates left positioned tree
                let left_child = layout_rec(left, ord, depth + 1);
                // calculates ord of current node
                let my_ord = if let PositionedTree::Node {
                    value: _,
                    left: _,
                    right,
                    x: left_child_ord,
                    y: _,
                } = &left_child
                {
                    if let PositionedTree::Node {
                        value: _,
                        left: _,
                        right: _,
                        x: left_grandchild_ord,
                        y: _,
                    } = right.as_ref()
                    {
                        left_grandchild_ord + 1
                    } else {
                        left_child_ord + 1
                    }
                } else {
                    ord
                };
                // creates right positioned tree
                let right_child = layout_rec(right.as_ref(), my_ord + 1, depth + 1);

                PositionedTree::node(*value, left_child, right_child, my_ord, depth)
            }
            Tree::End => PositionedTree::end(),
        }
    }

    layout_rec(tree, 1, 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use P57::from_list;
    #[test]
    fn test_layout_bintree() {
        let tree = Tree::<char>::end();
        let expected = PositionedTree::<char>::end();
        assert_eq!(layout_bintree(&tree), expected);

        let tree = Tree::leaf('a');
        let expected = PositionedTree::leaf('a', 1, 1);
        assert_eq!(layout_bintree(&tree), expected);

        let tree = Tree::node(
            'a',
            Tree::node('b', Tree::end(), Tree::leaf('c')),
            Tree::leaf('d'),
        );
        let expected = PositionedTree::node(
            'a',
            PositionedTree::node(
                'b',
                PositionedTree::end(),
                PositionedTree::leaf('c', 2, 3),
                1,
                2,
            ),
            PositionedTree::leaf('d', 4, 2),
            3,
            1,
        );
        assert_eq!(layout_bintree(&tree), expected);

        let tree = from_list(&vec![
            'n', 'k', 'm', 'c', 'a', 'h', 'g', 'e', 'u', 'p', 's', 'q',
        ]);
        assert_eq!(layout_bintree(&tree).to_string(), "T[8,1](n T[6,2](k T[2,3](c T[1,4](a . .) T[5,4](h T[4,5](g T[3,6](e . .) .) .)) T[7,3](m . .)) T[12,2](u T[9,3](p . T[11,4](s T[10,5](q . .) .)) .))");
    }
}
