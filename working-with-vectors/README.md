## Working with Vectors and Iterators

Vector ([std::vec::Vec](https://doc.rust-lang.org/std/vec/struct.Vec.html)) is one of the most important collection types in Rust, which allows you to store more than one value in a single data structure that puts all the values next to each other in memory. Iterator ([std::iter::Iterator](https://doc.rust-lang.org/std/iter/trait.Iterator.html)) provides unified, elegant interfaces to iteratively process the items in a collection.

Before working on this section, it is highly recommended that you read "[Storing Lists of Values with Vectors](https://doc.rust-lang.org/book/ch08-01-vectors.html)" and "[Processing a Series of Items with Iterators](https://doc.rust-lang.org/book/ch13-02-iterators.html)" in [The Rust Programming Language](https://doc.rust-lang.org/book/title-page.html) (a.k.a. "The Book").

Also you may want to consult with the documentation below to find proper usage of the standard library:

- https://doc.rust-lang.org/std/vec/struct.Vec.html
- https://doc.rust-lang.org/std/iter/trait.Iterator.html

### [P01](./P01/src/lib.rs) (*) Find the last element of a list.

Example: [examples/last.rs](./P01/examples/last.rs)
```rust
let li = vec![1, 1, 2, 3, 5, 8];
println!("{:?}", last(&li));
```

```bash
P01 $ cargo run -q --example last
Some(8)
```

### [P02](./P02/src/lib.rs) (*) Find the last but one element of a list.

Example: [examples/penultimate.rs](./P02/examples/penultimate.rs)
```rust
let li = vec![1, 1, 2, 3, 5, 8];
println!("{:?}", penultimate(&li));
```

```bash
P02 $ cargo run -q --example penultimate
Some(5)
```

### [P03](./P03/src/lib.rs) (*) Find the Kth element of a list.

Example: [examples/nth.rs](./P03/examples/nth.rs)
```rust
let li = vec![1, 1, 2, 3, 5, 8];
println!("{:?}", nth(2, &li));
```

```bash
P03 $ cargo run -q --example nth
Some(2)
```

### [P04](./P04/src/lib.rs) (*) Find the number of elements of a list.

Example: [examples/length.rs](./P04/examples/length.rs)
```rust
let li = vec![1, 1, 2, 3, 5, 8];
println!("{:?}", length(&li));
```

```bash
P04 $ cargo run -q --example length
6
```

### [P05](./P05/src/lib.rs) (*) Reverse a list.

Example: [examples/reverse.rs](./P05/examples/reverse.rs)
```rust
let li = vec![1, 1, 2, 3, 5, 8];
println!("{:?}", reverse(&li));
```

```bash
P05 $ cargo run -q --example reverse
[8, 5, 3, 2, 1, 1]
```

### [P06](./P06/src/lib.rs) (*) Find out whether a list is a palindrome.

Example: [examples/palindrome.rs](./P06/examples/palindrome.rs)
```rust
let f = |li| {
    println!("{:?} is {} a palindrome.", li, if is_palindrome(&li) { "" } else { "not" });
};

let li = vec![1, 2, 3, 2, 1];
f(li);

let li = vec![1, 3, 3, 2, 1];
f(li);
```

```bash
P06 $ cargo run -q --example palindrome
[1, 2, 3, 2, 1] is  a palindrome.
[1, 3, 3, 2, 1] is not a palindrome.
```

### [P07](./P07/src/lib.rs) (**) Flatten a nested list structure.

Problem Setup:
```rust
#[derive(Debug, PartialEq)]
pub struct List<T: Copy>(Vec<ListItem<T>>);

#[derive(Debug, PartialEq)]
pub enum ListItem<T: Copy> {
    Item(T),
    NestedItem(Box<List<T>>),
}

pub mod utils {
    use super::{List, ListItem};
    pub fn list<T: Copy>(items: Vec<ListItem<T>>) -> List<T> {
        List(items)
    }
    pub fn item<T: Copy>(v: T) -> ListItem<T> {
        ListItem::Item(v)
    }
    pub fn nested<T: Copy>(li: List<T>) -> ListItem<T> {
        ListItem::NestedItem(Box::new(li))
    }
}
```

Example: [examples/flatten.rs](./P07/examples/flatten.rs)
```rust
let mut li = list(vec![
    nested(list(vec![item(1), item(1)])),
    item(2),
    nested(list(vec![item(3), nested(list(vec![item(5), item(8)]))])),
]);
println!("{:?}", flatten(&li));
```

```bash
P07 $ cargo run -q --example flatten
List([Item(1), Item(1), Item(2), Item(3), Item(5), Item(8)])
```

### [P08](./P08/src/lib.rs) (**) Eliminate consecutive duplicates of list elements.

If a list contains repeated elements they should be replaced with a single copy of the element. The order of the elements should not be changed. 

Example: [examples/compress.rs](./P08/examples/compress.rs)
```rust
let li = vec!['a', 'a', 'a', 'a', 'b', 'c', 'c', 'a', 'a', 'd', 'e', 'e', 'e', 'e'];
println!("{:?}", compress(&li));
```

```bash
P08 $ cargo run -q --example compress
['a', 'b', 'c', 'a', 'd', 'e']
```

### [P09](./P09/src/lib.rs) (**) Pack consecutive duplicates of list elements into sublists.

If a list contains repeated elements they should be placed in separate sublists.

Example: [examples/pack.rs](./P09/examples/pack.rs)
```rust
let li = vec!['a', 'a', 'a', 'a', 'b', 'c', 'c', 'a', 'a', 'd', 'e', 'e', 'e', 'e'];
println!("{:?}", pack(&li));
```

```bash
P09 $ cargo run -q --example pack
[['a', 'a', 'a', 'a'], ['b'], ['c', 'c'], ['a', 'a'], ['d'], ['e', 'e', 'e', 'e']]
```

### [P10](./P10/src/lib.rs) (*) Run-length encoding of a list.

Use the result of problem P09 to implement the so-called run-length encoding data compression method. Consecutive duplicates of elements are encoded as tuples (N, E) where N is the number of duplicates of the element E.

Example: [examples/encode.rs](./P10/examples/encode.rs)
```rust
let li = vec!['a', 'a', 'a', 'a', 'b', 'c', 'c', 'a', 'a', 'd', 'e', 'e', 'e', 'e'];
println!("{:?}", encode(&li));
```

```bash
P10 $ cargo run -q --example encode
[(4, 'a'), (1, 'b'), (2, 'c'), (2, 'a'), (1, 'd'), (4, 'e')]
```

### [P11](./P11/src/lib.rs) (*) Modified run-length encoding.

Omitted;

### [P12](./P12/src/lib.rs) (**) Decode a run-length encoded list.

Given a run-length code list generated as specified in problem P10, construct its uncompressed version. 

Example: [examples/decode.rs](./P12/examples/decode.rs)
```rust
let li = vec![(4, 'a'), (1, 'b'), (2, 'c'), (2, 'a'), (1, 'd'), (4, 'e')];
println!("{:?}", decode(&li));
```

```bash
P12 $ cargo run -q --example decode
['a', 'a', 'a', 'a', 'b', 'c', 'c', 'a', 'a', 'd', 'e', 'e', 'e', 'e']
```

### [P13](./P13/src/lib.rs) (**) Run-length encoding of a list (direct solution).

Implement the so-called run-length encoding data compression method directly. I.e. don't use other methods you've written (like P09's pack); do all the work directly. 

Example: [examples/encode_direct.rs](./P13/examples/encode_direct.rs)
```rust
let li = vec!['a', 'a', 'a', 'a', 'b', 'c', 'c', 'a', 'a', 'd', 'e', 'e', 'e', 'e'];
println!("{:?}", encode_direct(&li));
```

```bash
P13 $ cargo run -q --example encode_direct
[(4, 'a'), (1, 'b'), (2, 'c'), (2, 'a'), (1, 'd'), (4, 'e')]
```

### [P14](./P14/src/lib.rs) (*) Duplicate the elements of a list.

Example: [examples/duplicate.rs](./P14/examples/duplicate.rs)
```rust
let li = vec!['a', 'b', 'c', 'c', 'd'];
println!("{:?}", duplicate(&li));
```

```bash
P14 $ cargo run -q --example duplicate
['a', 'a', 'b', 'b', 'c', 'c', 'c', 'c', 'd', 'd']
```

### [P15](./P15/src/lib.rs) (**) Duplicate the elements of a list a given number of times.

Example: [examples/duplicate_n.rs](./P15/examples/duplicate_n.rs)
```rust
let li = vec!['a', 'b', 'c', 'c', 'd'];
println!("{:?}", duplicate_n(3, &li));
```

```bash
P15 $ cargo run -q --example duplicate_n
['a', 'a', 'a', 'b', 'b', 'b', 'c', 'c', 'c', 'c', 'c', 'c', 'd', 'd', 'd']
```

### [P16](./P16/src/lib.rs) (**) Drop every Nth element from a list.

Example: [examples/drop.rs](./P16/examples/drop.rs)
```rust
let li = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k'];
println!("{:?}", drop(3, &li));
```

```bash
P16 $ cargo run -q --example drop
['a', 'b', 'd', 'e', 'g', 'h', 'j', 'k']
```

### [P17](./P17/src/lib.rs) (*) Split a list into two parts.

The length of the first part is given. Use a Tuple for your result.

Example: [examples/split.rs](./P17/examples/split.rs)
```rust
let li = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k'];
println!("{:?}", split(3, &li));
```

```bash
P17 $ cargo run -q --example split
(['a', 'b', 'c'], ['d', 'e', 'f', 'g', 'h', 'i', 'j', 'k'])
```

### [P18](./P18/src/lib.rs) (**) Extract a slice from a list.

Given two indices, _I_ and _K_, the slice is the list containing the elements from and including the _I_ th element up to but not including the _K_ th element of the original list. Start counting the elements with 0. 

Example: [examples/slice.rs](./P18/examples/slice.rs)
```rust
let li = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k'];
println!("{:?}", slice(3, 7, &li));
```

```bash
P18 $ cargo run -q --example slice
['d', 'e', 'f', 'g']
```

### [P19](./P19/src/lib.rs) (**) Rotate a list N places to the left.

Example: [examples/rotate.rs](./P19/examples/rotate.rs)
```rust
let li = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k'];
println!("rotate on position 3: {:?}", rotate(3, &li));
println!("rotate on position -2: {:?}", rotate(-2, &li));
```

```bash
P19 $ cargo run -q --example rotate
rotate on position 3: ['d', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'a', 'b', 'c']
rotate on position -2: ['j', 'k', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i']
```

### [P20](./P20/src/lib.rs) (*) Remove the Kth element from a list.

Return the list and the removed element in a Tuple. Elements are numbered from 0.

Example: [examples/remove_at.rs](./P20/examples/remove_at.rs)
```rust
let li = vec!['a', 'b', 'c', 'd'];
println!("remove element at position 1: {:?}", remove_at(1, &li));
```

```bash
P20 $ cargo run -q --example remove_at
remove element at position 1: (['a', 'c', 'd'], 'b')
```

### [P21](./P21/src/lib.rs) (*) Insert an element at a given position into a list.

Example: [examples/insert_at.rs](./P21/examples/insert_at.rs)
```rust
let li = vec!['a', 'b', 'c', 'd'];
println!("insert 'n' at position 1: {:?}", insert_at('n', 1, &li));
```

```bash
P21 $ cargo run -q --example insert_at
insert 'n' at position 1: ['a', 'n', 'b', 'c', 'd']
```

### [P22](./P22/src/lib.rs) (*) Create a list containing all integers within a given range.

Example: [examples/range.rs](./P22/examples/range.rs)
```rust
println!("{:?}", range(4, 9));
```

```bash
P22 $ cargo run -q --example range
[4, 5, 6, 7, 8, 9]
```

### [P23](./P23/src/lib.rs) (**)  Extract a given number of randomly selected elements from a list.

Hint: Use [rand](https://crates.io/crates/rand) crate

Example: [examples/random_select.rs](./P23/examples/random_select.rs)
```rust
let li = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
println!("{:?}", random_select(3, &li));
```

```bash
P23 $ cargo run -q --example random_select
['d', 'b', 'a']
```

### [P24](./P24/src/lib.rs) (*) Lotto: Draw N different random numbers from the set 1..M.

Example: [example/lotto.rs](./P24/examples/lotto.rs)
```rust
println!("{:?}", lotto(6, 49));
println!("{:?}", lotto(6, 49));
println!("{:?}", lotto(6, 49));
```

```bash
P24 $ cargo run -q --example lotto
[43, 12, 5, 31, 37, 27]
[29, 31, 10, 4, 19, 17]
[40, 41, 49, 18, 36, 21]
```

### [P25](./P25/src/lib.rs) (*) Generate a random permutation of the elements of a list.

Hint: Use the solution of problem P23.

Example: [examples/permute.rs](./P25/examples/permute.rs)
```rust
let li = vec!['a', 'b', 'c', 'd', 'e', 'f'];
println!("{:?}", random_permute(&li));
println!("{:?}", random_permute(&li));
println!("{:?}", random_permute(&li));
```

```bash
P25 $ cargo run -q --example permute
['c', 'b', 'e', 'f', 'a', 'd']
['d', 'f', 'b', 'e', 'a', 'c']
['f', 'c', 'e', 'b', 'd', 'a']
```

### [P26](./P26/src/lib.rs) (**) Generate the combinations of K distinct objects chosen from the N elements of a list.

In how many ways can a committee of 3 be chosen from a group of 12 people? We all know that there are C(12,3) = 220 possibilities (C(N,K) denotes the well-known binomial coefficient). For pure mathematicians, this result may be great. But we want to really generate all the possibilities. 

Example: [examples/combinations.rs](./P26/examples/combinations.rs)
```rust
let li = vec!['a', 'b', 'c', 'd', 'e', 'f'];
println!("{:?}", combinations(3, &li));
```

```bash
P26 $ cargo run -q --example combinations
[['a', 'b', 'c'], ['a', 'b', 'd'], ['a', 'b', 'e'], ['a', 'b', 'f'], ['a', 'c', 'd'], ['a', 'c', 'e'], ['a', 'c', 'f'], ['a', 'd', 'e'], ['a', 'd', 'f'], ['a', 'e', 'f'], ['b', 'c', 'd'], ['b', 'c', 'e'], ['b', 'c', 'f'], ['b', 'd', 'e'], ['b', 'd', 'f'], ['b', 'e', 'f'], ['c', 'd', 'e'], ['c', 'd', 'f'], ['c', 'e', 'f'], ['d', 'e', 'f']]
```

### [P27](./P27/src/lib.rs) (**) Group the elements of a set into disjoint subsets.

a) In how many ways can a group of 9 people work in 3 disjoint subgroups of 2, 3 and 4 persons? Write a function that generates all the possibilities. 

Example: [examples/group3.rs](./P27/examples/group3.rs)
```rust
let li = vec![
    "Aldo", "Beat", "Carla", "David", "Evi", "Flip", "Gary", "Hugo", "Ida",
];
let groups = group3(&li);
groups.into_iter().for_each(|x| println!("{:?}", x));
```

```bash
P27 $ cargo run -q --example group3
[["Aldo", "Beat"], ["Carla", "David", "Evi"], ["Flip", "Gary", "Hugo", "Ida"]]
[["Aldo", "Beat"], ["Carla", "David", "Flip"], ["Evi", "Gary", "Hugo", "Ida"]]
[["Aldo", "Beat"], ["Carla", "David", "Gary"], ["Evi", "Flip", "Hugo", "Ida"]]
...
```

b) Generalize the above predicate in a way that we can specify a list of group sizes and the predicate will return a list of groups.

Example: [examples/group.rs](./P27/examples/group.rs)
```rust
let li = vec![
    "Aldo", "Beat", "Carla", "David", "Evi", "Flip", "Gary", "Hugo", "Ida",
];
let groups = group(&[2, 2, 5], &li);
groups.into_iter().for_each(|x| println!("{:?}", x));
```

```bash
P27 $ cargo run -q --example group
[["Aldo", "Beat"], ["Carla", "David"], ["Evi", "Flip", "Gary", "Hugo", "Ida"]]
[["Aldo", "Beat"], ["Carla", "Evi"], ["David", "Flip", "Gary", "Hugo", "Ida"]]
[["Aldo", "Beat"], ["Carla", "Flip"], ["David", "Evi", "Gary", "Hugo", "Ida"]]
...
```

### [P28](./P28/src/lib.rs) (**) Sorting a list of lists according to length of sublists.

a) We suppose that a list contains elements that are lists themselves. The objective is to sort the elements of the list according to their length. E.g. short lists first, longer lists later, or vice versa.

Example: [examples/lsort.rs](./P28/examples/lsort.rs)
```rust
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
```

```bash
P28 $ cargo run -q --example lsort
sorted=[['o'], ['d', 'e'], ['d', 'e'], ['m', 'n'], ['a', 'b', 'c'], ['f', 'g', 'h'], ['i', 'j', 'k', 'l']]
```

b) Again, we suppose that a list contains elements that are lists themselves. But this time the objective is to sort the elements according to their length frequency; i.e. in the default, sorting is done ascendingly, lists with rare lengths are placed, others with a more frequent length come later.

Example: [examples/lsort_freq.rs](./P28/examples/lsort_freq.rs)
```rust
let li = vec![
    vec!['a', 'b', 'c'],
    vec!['d', 'e'],
    vec!['f', 'g', 'h'],
    vec!['d', 'e'],
    vec!['i', 'j', 'k', 'l'],
    vec!['m', 'n'],
    vec!['o'],
];
println!("sorted={:?}", lsort_freq(&li));
```

```bash
P28 $ cargo run -q --example lsort_freq
sorted=[['o'], ['i', 'j', 'k', 'l'], ['a', 'b', 'c'], ['f', 'g', 'h'], ['d', 'e'], ['d', 'e'], ['m', 'n']]
```