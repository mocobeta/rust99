use bintree::Tree;
use bintree_pos::PositionedTree;
use std::fmt;

pub fn layout_bintree2<T: Copy + fmt::Display>(tree: &Tree<T>) -> PositionedTree<T> {
    fn layout_rec<T: Copy + fmt::Display>(
        node: &Tree<T>,
        xpos: u32,
        depth: u32,
        hdist: u32,
    ) -> PositionedTree<T> {
        match node {
            Tree::Node { value, left, right } => {
                let left = layout_rec(left, xpos - hdist, depth + 1, hdist / 2);
                let right = layout_rec(right, xpos + hdist, depth + 1, hdist / 2);
                PositionedTree::node(*value, left, right, xpos, depth)
            }
            Tree::End => PositionedTree::end(),
        }
    }

    /// Calculate horizontal distance between a parent and child node
    fn hdist(height: usize) -> usize {
        2u32.pow(height as u32 - 1) as usize
    }

    /// Calculate the horizontal position of root node
    fn root_xpos<T: fmt::Display>(tree: &Tree<T>, xpos: u32, height: usize) -> u32 {
        match tree {
            Tree::Node {
                value: _,
                left,
                right: _,
            } => match left.as_ref() {
                Tree::Node { .. } => root_xpos(left, xpos, height - 1) + hdist(height) as u32,
                Tree::End => 1,
            },
            Tree::End => 0,
        }
    }

    match tree {
        Tree::Node {
            value,
            left: _,
            right: _,
        } => {
            if tree.height() == 1 {
                PositionedTree::leaf(*value, 1, 1)
            } else {
                let xpos = root_xpos(&tree, 0, tree.height() - 1);
                let hdist = hdist(tree.height() - 1) as u32;
                layout_rec(tree, xpos, 1, hdist)
            }
        }
        Tree::End => PositionedTree::end(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use P57::from_list;

    #[test]
    fn test_layout_bintree2() {
        let tree = Tree::<char>::end();
        let expected = PositionedTree::<char>::end();
        assert_eq!(layout_bintree2(&tree), expected);

        let tree = Tree::leaf('a');
        let expected = PositionedTree::leaf('a', 1, 1);
        assert_eq!(layout_bintree2(&tree), expected);

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
            PositionedTree::leaf('d', 5, 2),
            3,
            1,
        );
        assert_eq!(layout_bintree2(&tree), expected);

        let tree = from_list(&vec!['n', 'k', 'm', 'c', 'a', 'e', 'd', 'g', 'u', 'p', 'q']);
        assert_eq!(layout_bintree2(&tree).to_string(), "T[15,1](n T[7,2](k T[3,3](c T[1,4](a . .) T[5,4](e T[4,5](d . .) T[6,5](g . .))) T[11,3](m . .)) T[23,2](u T[19,3](p . T[21,4](q . .)) .))");
    }
}
