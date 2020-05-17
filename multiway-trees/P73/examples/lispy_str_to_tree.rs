use P73::*;

pub fn main() {
    let s = "(a (f g) c (b d e))";
    println!("{:?}", lispy_str_to_tree(&s));
}
