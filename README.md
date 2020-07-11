# [WIP] R-99: Ninety-Nine Rust Problems

## What is this repository?

This is an adoption of the [S-99: Ninety-Nine Scala Problems](http://aperiodic.net/phil/scala/s-99/) or the [P-99: Ninety-Nine Prolog Problems](https://www.ic.unicamp.br/~meidanis/courses/mc336/2009s2/prolog/problemas/) for [Rust Programming Language](https://www.rust-lang.org/). I found that the problem colleciton is also good for Rust learners, so I started to rewrite them with Rustlang. This project is mainly for my learning and practicing, but I hope it is also useful for other Rustaceans.

## Usage of this collection

(The descrioption / instruction below were partially cited from the original sites, [S-99](http://aperiodic.net/phil/scala/s-99/) and [P-99](https://www.ic.unicamp.br/~meidanis/courses/mc336/2009s2/prolog/problemas/).)

The purpose of this problem collection is to give you the opportunity to practice your skills in Rust programming and algorithms. Your goal should be to find the most elegant solution of the given problems. Efficiency is important, but logical clarity is even more crucial. Some of the (easy) problems can be trivially solved using built-in functions. However, in these cases, you learn more if you try to find your own solution. 

The problems have different levels of difficulty. Those marked with a single asterisk (\*) are easy. If you have successfully solved the preceeding problems you should be able to solve them within a few (say 15) minutes. Problems marked with two asterisks (\*\*) are of intermediate difficulty. If you are a skilled Rust programmer it shouldn't take you more than 30-90 minutes to solve them. Problems marked with three asterisks (\*\*\*) are more difficult. You may need more time (i.e. a few hours or more) to find a good solution. The difficulties were all assigned for the Prolog problems, but the Rust versions seem to be of roughly similar difficulty.

For most (but not all) problems solutions are available via the link at the beginning of the problem description. Each solution is an independent lib crate with unit tests and examples so you can compile and execute them by `cargo build`, `cargo test` and `cargo run --example <filename>` command.

Although I am careful about doing this stuff, there may be bugs as a software product. Also, provided problem setups or solutions here would not be the best ones in terms of efficiency and elegancy. Feedback is appreciated. 

## The Rust Toolchain version currently used

See [rust-toolchain](./rust-toolchain) file.

## Sections

The collection consists of a few sections; let's exlore them from the following links!

- (P01-P28) [Working with Vectors and Iterators](./working-with-vectors/README.md)
- (P31-P41) [Arithmetic](./arithmetic/README.md)
- (P46-P50) [Logic and Codes](./logic-and-codes/README.md)
- (P55-P69) [Binary Trees](./binary-trees/README.md)
- (P70-P73) [Multiway Trees](./multiway-trees/README.md)
- (P80-P89) [Graphs](./graphs/README.md)
- (TBD) Miscellaneous Problems








