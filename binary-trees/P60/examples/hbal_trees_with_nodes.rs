use P60::*;

pub fn main() {
    let hbal_trees = hbal_trees_with_nodes(4, 'x');
    for tree in hbal_trees {
        println!("{}", tree)
    }
}
