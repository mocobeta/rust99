use P47::*;
use P48::*;

pub fn main() {
    let f = |args: &[bool]| -> bool {
        let a = args.get(0).unwrap();
        let b = args.get(1).unwrap();
        let c = args.get(2).unwrap();
        a.and(&b.or(c)).equ(&a.and(&b).or(&a.and(&c)))
    };
    let truth_tbl = table(3, f);

    println!("a\tb\tc\tresult");
    for v in truth_tbl {
        let a = v.get(0).unwrap();
        let b = v.get(1).unwrap();
        let c = v.get(2).unwrap();
        let result = v.get(3).unwrap();
        println!("{}\t{}\t{}\t{}", a, b, c, result);
    }
}
