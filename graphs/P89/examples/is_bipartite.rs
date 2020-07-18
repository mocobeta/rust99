use P80::graph_converters::unlabeled;
use P89::*;

pub fn main() {
    let g = unlabeled::from_string("[a-b, b-c, d, e-f, f-g, g-e, h]");
    println!(
        "{:?} is bipartite: {}",
        unlabeled::to_term_form(&g),
        is_bipartite(&g)
    );

    let g = unlabeled::from_string("[a-b, b-c, d, e-f, g-e, h]");
    println!(
        "{:?} is bipartite: {}",
        unlabeled::to_term_form(&g),
        is_bipartite(&g)
    );
}
