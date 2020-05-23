use P80::digraph_converters::{labeled, unlabeled};

pub fn main() {
    let g = unlabeled::from_string("[s>r, t, u>r, s>u, u>s, v>u]");
    println!(
        "unlabeled digraph (graph-term form)\n{:?}",
        unlabeled::to_term_form(&g)
    );

    let g = labeled::from_string("[p>q/9, m>q/7, k, p>m/5]");
    println!(
        "labeled digraph (adjacency-list form)\n{:?}",
        labeled::to_adjacent_form(&g)
    );
}
