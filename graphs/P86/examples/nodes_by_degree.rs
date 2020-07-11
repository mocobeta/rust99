use P80::graph_converters::unlabeled;

pub fn main() {
    let g = unlabeled::from_string("[a-b, b-c, a-c, a-d]");
    println!("{:?}", g.get_nodes_by_degree(true));
}
