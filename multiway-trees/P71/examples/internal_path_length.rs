use P70::str_to_tree;
use P71::*;

pub fn main() {
    println!("{}", internal_path_length(&str_to_tree("afg^^c^bd^e^^^")));
}
