use P56::*;
use P57::*;

pub fn main() {
    let values = vec![3, 2, 5, 7, 1];
    let tree = from_list(&values);
    println!("from {:?}: {}", values, tree);

    println!("----");

    let values = vec![5, 3, 18, 1, 4, 12, 21];
    let tree = from_list(&values);
    println!(
        "from_list({:?}) is {}.",
        values,
        if is_symmetric(&tree) {
            "symmetric"
        } else {
            "not symmetric"
        }
    );

    let values = vec![3, 2, 5, 7, 4];
    let tree = from_list(&values);
    println!(
        "from_list({:?}) is {}.",
        values,
        if is_symmetric(&tree) {
            "symmetric"
        } else {
            "not symmetric"
        }
    );
}
