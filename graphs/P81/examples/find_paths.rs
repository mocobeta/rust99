use P80::digraph_converters::labeled;
use P81::*;

pub fn main() {
    let g = labeled::from_string("[p>q/9, m>q/7, k, p>m/5]");
    println!("Paths from p to q: {:?}", g.find_paths('p', 'q'));
    println!("Paths from p to k: {:?}", g.find_paths('p', 'k'));
}
