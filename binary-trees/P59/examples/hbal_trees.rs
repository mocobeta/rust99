use P59::*;

pub fn main() {
    let trees = hbal_trees(3, 'x');
    for tree in trees {
        println!("{}", tree);
    }
}
