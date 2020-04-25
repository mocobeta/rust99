## Logic and Codes

### [P46](./P46/src/lib.rs) (**) Truth tables for logical expressions.

Define functions and, or, not, nand, nor, xor, impli[1], and equ[2] (for logical equivalence) which return true or false according to the result of their respective operations; e.g. and(a, b) is true if and only if both _a_ and _b_ are true.

[1] implication https://en.wikipedia.org/wiki/Material_conditional

[2] equality https://en.wikipedia.org/wiki/Logical_equality

Example: [examples/logical_functions.rs](./P46/examples/logical_functions.rs)
```rust
    println!("and(true,false)={}", and(true, false));
    println!("or(true,false)={}", or(true, false));
    println!("not(true)={}", not(true));
    println!("nand(true,false)={}", nand(true, false));
    println!("nor(true,false)={}", nor(true, false));
    println!("xor(true,false)={}", xor(true, false));
    println!("impli(true,false)={}", impli(true, false));
    println!("equ(true,false)={}", equ(true, false));
```

```bash
P46 $ cargo run -q --example logical_functions
and(true,false)=false
or(true,false)=true
not(true)=false
nand(true,false)=true
nor(true,false)=false
xor(true,false)=true
impli(true,false)=false
equ(true,false)=false
```

A logical expression in two variables can then be written as a function or closure of two variables, e.g: |a, b| and(or(a, b), nand(a, b))

Now, write a function called table2 which prints the truth table of a given logical expression in two variables.

Example: [examples/table2.rs](./P46/examples/table2.rs)
```rust
let f = |a, b| and(a, or(a, b));
let truth_tbl = table2(f);
println!("a\tb\tresult");
for (a, b, result) in truth_tbl {
    println!("{}\t{}\t{}", a, b, result);
}
```

```bash
P46 $ cargo run -q --example table2
a       b       result
true    true    true
true    false   true
false   true    false
false   false   false
```

### [P47](./P47/src/lib.rs) (*) Truth tables for logical expressions (2).

Continue problem P46 by defining and, or, etc as being operators. This allows to write the logical expression in the more natural way, as in the example: `a.and(&a.or(&not(b))`. Define operator precedence as usual.

Hint: You can define a Trait to implement custom operators for `bool`.

Problem Setup:
```rust
pub trait LogicalOps {
    fn and(&self, other: &bool) -> bool;
    fn or(&self, other: &bool) -> bool;
    ...
}
impl LogicalOps for bool {
    fn and(&self, other: &bool) -> bool {
        // FILL THIS METHOD
    }
    fn or(&self, other: &bool) -> bool {
        // FILL THIS METHOD
    }
}
```

Example: [examples/table2_op.rs](./P47/examples/table2_op.rs)
```rust
let f = |a: bool, b: bool| a.and(&a.or(&not(b)));
let truth_tbl = table2(f);
println!("a\tb\tresult");
for (a, b, result) in truth_tbl {
    println!("{}\t{}\t{}", a, b, result);
}
```

```bash
P47 $ cargo run -q --example table2_op
a       b       result
true    true    true
true    false   true
false   true    false
false   false   false
```

### [P48](./P48/src/lib.rs) (**) Truth tables for logical expressions (3).

Generalize problem P47 in such a way that the logical expression may contain any number of logical variables. Define table in a way that table(k,expr) prints the truth table for the expression `expr`, which contains `k` logical variables.

Example: [examples/table.rs](./P48/examples/table.rs)
```rust
let f = |args: Vec<bool>| -> bool {
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
```

```bash
P48 $ cargo run -q --example table
a       b       c       result
true    true    true    true
true    true    false   true
true    false   true    true
true    false   false   true
false   true    true    true
false   true    false   true
false   false   true    true
false   false   false   true
```

### [P49](./P49/src/lib.rs) (**) Gray code.

An n-bit Gray code is a sequence of n-bit strings constructed according to certain rules. For example,

```
n = 1: C(1) = ("0", "1").
n = 2: C(2) = ("00", "01", "11", "10").
n = 3: C(3) = ("000", "001", "011", "010", "110", "111", "101", "100")
```

Find out the construction rules and write a function to generate Gray codes. See if you can use memoization to make the function more efficient.

Example: [examples/gray.rs](./P49/examples/gray.rs)
```rust
println!("n=3: {:?}", gray(3));
```

```bash
P49 $ cargo run -q --example gray
n=3: ["000", "001", "011", "010", "110", "111", "101", "100"]
```

### [P50](./P50/src/lib.rs) (***) Huffman code.

First of all, consult a good book on discrete mathematics or algorithms for a detailed description of Huffman codes!

We suppose a set of symbols with their frequencies, given as a list of (S, F) Tuples. E.g. `vec![('a', 45), ('b', 13), ('c', 12), ('d', 16), ('e', 9), ('f', 5)]`. Our objective is to construct a list of (S, C) Tuples, where C is the Huffman code word for the symbol S.

Hint: See [std::collections::BinaryHeap](https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html) to build the Huffman tree.

Example: [examples/huffman.rs](./P50/examples/huffman.rs)
```rust
    let symbols = vec![
        ('a', 45),
        ('b', 13),
        ('c', 12),
        ('d', 16),
        ('e', 9),
        ('f', 5),
    ];
    println!("{:?}", huffman(&symbols));
```

```bash
P50 $ cargo run -q --example huffman
[('a', "0"), ('b', "101"), ('c', "100"), ('d', "111"), ('e', "1101"), ('f', "1100")]
```