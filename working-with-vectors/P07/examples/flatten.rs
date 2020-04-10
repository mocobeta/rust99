use P07::flatten;
use P07::utils::*;

pub fn main() {
    let li = list(vec![
        nested(list(vec![item(1), item(1)])),
        item(2),
        nested(list(vec![item(3), nested(list(vec![item(5), item(8)]))])),
    ]);
    println!("{:?}", flatten(&li));
}
