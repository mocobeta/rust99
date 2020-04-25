use P46::*;

pub fn main() {
    let f = |a, b| and(a, or(a, b));
    let truth_tbl = table2(f);

    println!("a\tb\tresult");
    for (a, b, result) in truth_tbl {
        println!("{}\t{}\t{}", a, b, result);
    }
}
