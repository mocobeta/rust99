use P80::graph_converters::unlabeled;
use P85::*;

pub fn main() {
    let g1 = unlabeled::from_string("[a-b]");
    let g2 = unlabeled::from_string("[5-7]");
    println!(
        "{:?} is isomorphic to {:?}: {}",
        unlabeled::to_term_form(&g1),
        unlabeled::to_term_form(&g2),
        is_isomorphic_to(&g1, &g2)
    );
}
