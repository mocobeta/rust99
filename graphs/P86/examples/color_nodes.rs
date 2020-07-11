use P80::graph_converters::unlabeled;
use P86::*;

pub fn main() {
    let g = unlabeled::from_string("[a-b, b-c, a-c, a-d]");
    println!("{:?}", color_nodes(&g));
}
