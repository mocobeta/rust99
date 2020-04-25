use P46::*;
use P47::*;

pub fn main() {
    let f = |a: bool, b: bool| a.and(&a.or(&not(b)));
    let truth_tbl = table2(f);

    println!("a\tb\tresult");
    for (a, b, result) in truth_tbl {
        println!("{}\t{}\t{}", a, b, result);
    }
}
