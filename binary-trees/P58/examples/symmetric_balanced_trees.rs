use P58::*;

pub fn main() {
    let trees = symmetric_balanced_trees(5, 'x');
    for tree in trees {
        println!("{}", tree);
    }
}
