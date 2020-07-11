use P80::graph_converters::unlabeled;
use P83::*;

pub fn main() {
    let g = unlabeled::from_term_form(
        &vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'],
        &vec![
            ('a', 'b'),
            ('a', 'd'),
            ('b', 'c'),
            ('b', 'e'),
            ('c', 'e'),
            ('d', 'e'),
            ('d', 'f'),
            ('d', 'g'),
            ('e', 'h'),
            ('f', 'g'),
            ('g', 'h'),
        ],
    );
    let trees = spanning_trees(&g);
    for tree in trees {
        println!("{:?}", unlabeled::to_term_form(&tree));
    }
}
