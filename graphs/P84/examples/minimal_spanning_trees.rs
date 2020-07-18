use P80::graph_converters::labeled;
use P84::*;

pub fn main() {
    let g = labeled::from_term_form(
        &vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'],
        &vec![
            ('a', 'b', 5),
            ('a', 'd', 3),
            ('b', 'c', 2),
            ('b', 'e', 4),
            ('c', 'e', 6),
            ('d', 'e', 7),
            ('d', 'f', 4),
            ('d', 'g', 3),
            ('e', 'h', 5),
            ('f', 'g', 4),
            ('g', 'h', 1),
        ],
    );
    let trees = minimal_spanning_trees(&g);
    for tree in trees {
        println!(
            "{:?} (weight={})",
            labeled::to_term_form(&tree),
            label_sum(&tree)
        );
    }
}
