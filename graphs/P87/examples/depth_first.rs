use P80::graph_converters::unlabeled;
use P87::*;

pub fn main() {
    let g = unlabeled::from_string("[a-b, b-c, e, a-c, a-d]");
    println!("{:?}", nodes_by_depth_from(&g, 'd'));
}
