use P55::*;

pub fn main() {
    let trees = cbal_trees(4, 'x');
    for tree in trees {
        println!("{}", tree);
    }
}
