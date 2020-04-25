use P46::*;

pub fn main() {
    println!("and(true,false)={}", and(true, false));
    println!("or(true,false)={}", or(true, false));
    println!("not(true)={}", not(true));
    println!("nand(true,false)={}", nand(true, false));
    println!("nor(true,false)={}", nor(true, false));
    println!("xor(true,false)={}", xor(true, false));
    println!("impli(true,false)={}", impli(true, false));
    println!("equ(true,false)={}", equ(true, false));
}
