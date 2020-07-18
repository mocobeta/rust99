use P80::graph_converters::unlabeled;
use P88::*;

pub fn main() {
    let splitted = split_graph(&unlabeled::from_string("[a-b, b-c, d, e-f, f-g, g-e, h]"));
    for g in splitted {
        println!("{:?}", unlabeled::to_term_form(&g));
    }
}
