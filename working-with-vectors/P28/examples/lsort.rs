use P28::lsort;

pub fn main() {
    let li = vec![
        vec!['a', 'b', 'c'],
        vec!['d', 'e'],
        vec!['f', 'g', 'h'],
        vec!['d', 'e'],
        vec!['i', 'j', 'k', 'l'],
        vec!['m', 'n'],
        vec!['o'],
    ];
    println!("sorted={:?}", lsort(&li));
}
