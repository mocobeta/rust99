use P80::graph_converters::unlabeled;
use P82::*;

pub fn main() {
    let g = unlabeled::from_string("[b-c, f-c, g-h, d, f-b, k-f, h-g]");
    println!("Cycles starting at f: {:?}", g.find_cycles('f'));
    println!("Cycles starting at g: {:?}", g.find_cycles('g'));
}
