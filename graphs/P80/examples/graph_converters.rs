use P80::graph_converters::{labeled, unlabeled};

pub fn main() {
    let g = unlabeled::from_string("[b-c, f-c, g-h, d, f-b, k-f, h-g]");
    println!(
        "unlabeled graph (graph-term form)\n{:?}",
        unlabeled::to_term_form(&g)
    );

    let g = labeled::from_string("[k, m-p/5, m-q/7, p-q/9]");
    println!(
        "labeled graph (adjacency-list form)\n{:?}",
        labeled::to_adjacent_form(&g)
    );
}
