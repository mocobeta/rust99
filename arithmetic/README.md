## Arithmetic

### [P31](./P31/src/lib.rs) (**) Determine whether a given integer number is prime.

Hint: Use [memoization](https://en.wikipedia.org/wiki/Memoization) for efficient calculation.

Problem Setup:
```rust
pub struct Primes {
    // ... YOUR CODE ...
}

impl Primes {
    pub fn is_prime(&mut self, n: usize) -> bool {
        // ... YOUR CODE ...
    }
}
```

Examples: [examples/is_prime.rs](./P31/examples/is_prime.rs)
```rust
let mut primes = Primes::new();
println!("Is {} a prime number? : {}", 53, primes.is_prime(53));
println!("Is {} a prime number? : {}", 1957, primes.is_prime(1957));
```

```bash
P31 $ cargo run -q --example is_prime
Is 53 a prime number? : true
Is 1957 a prime number? : false
```

### [P32](./P32/src/lib.rs) (**) Determine the greatest common divisor of two positive integer numbers.

Hint: Use Euclid's algorithm.

Example: [examples/gcd.rs](./P32/examples/gcd.rs)
```rust
println!("{}", gcd(36, 63));
```

```bash
P32 $ cargo run -q --example gcd
9
```

### [P33](./P33/src/lib.rs) (*) Determine whether two positive integer numbers are coprime.

Hint: Two numbers are coprime if their greatest common divisor equals 1.

Example: [examples/coprime.rs](./P33/examples/coprime.rs)
```rust
println!("35 is coprime to 64? {}", is_coprime_to(35, 64));
println!("35 is coprime to 65? {}", is_coprime_to(35, 65));
```

```bash
P33 $ cargo run -q --example coprime
35 is coprime to 64? true
35 is coprime to 65? false
```

### [P34](./P34/src/lib.rs) (**) Calculate Euler's totient function phi(m).

Euler's so-called totient function phi(m) is defined as the number of positive integers r (1 <= r <= m) that are coprime to m.

Example: [examples/totient.rs](./P34/examples/totient.rs)
```rust
println!("{}", totient(10));
```

```bash
P34 $ cargo run -q --example totient
4
```

### [P35](./P35/src/lib.rs) (**) Determine the prime factors of a given positive integer.

Construct a flat list containing the prime factors in ascending order.

Hint: You could define `PrimeIterator` struct and implement [Iterator trait](https://doc.rust-lang.org/std/iter/trait.Iterator.html) for that to iterate prime numbers.

```rust
pub struct PrimeIterator {
    primes: Primes,
    next: u32,
}

impl Iterator for PrimeIterator {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let res = self.next;
        self.next += 1;
        while !self.primes.is_prime(self.next) {
            self.next += 1;
        }
        Some(res)
    }
}
```

Example: [examples/prime_factors.rs](./P35/examples/prime_factors.rs)
```rust
println!("{:?}", prime_factors(315));
```

```bash
P35 $ cargo run -q --example prime_factors
[3, 3, 5, 7]
```

### [P36](./P36/src/lib.rs) (**) Determine the prime factors of a given positive integer (2).

Construct a list containing the prime factors and their multiplicity.

Example: [examples/prime_factor_mult.rs](./P36/examples/prime_factor_mult.rs)
```rust
println!("{:?}", prime_factor_multiplicity(315));
```

```bash
P36 $ cargo run -q --example prime_factor_mult
[(3, 2), (5, 1), (7, 1)]
```

### [P37](./P37/src/lib.rs) (**) Calculate Euler's totient function phi(m) (improved).

See problem P34 for the definition of Euler's totient function. If the list of the prime factors of a number m is known in the form of problem P36 then the function phi(m) can be efficiently calculated as follows: Let [[p1, m1], [p2, m2], [p3, m3], ...] be the list of prime factors (and their multiplicities) of a given number m. Then phi(m) can be calculated with the following formula:

phi(m) = (p1-1)\*p1^(m1-1) \* (p2-1)\*p2^(m2-1) \* (p3-1)\*p3^(m3-1) \* ...

Note that _a_ ^ _b_ stands for the _b_ th power of _a_.

Example: [examples/totient.rs](./P37/examples/totient.rs)
```rust
println!("{}", totient(10));
```

```bash
P37 $ cargo run -q --example totient
4
```

### [P38](./P38/src/lib.rs) (*) Compare the two methods of calculating Euler's totient function.

Use the solutions of problems P34 and P37 to compare the algorithms. Try to calculate phi(123456) as an example and measure performance of the two solutions by [benchmark tests feature](https://doc.rust-lang.org/unstable-book/library-features/test.html).

`cargo bench` will show the test results like the following.

```bash
P38 $ cargo bench -q

running 2 tests
test tests::bench_p34_totient ... bench:   8,002,703 ns/iter (+/- 438,102)
test tests::bench_p37_totient ... bench:      66,033 ns/iter (+/- 3,334)
```

### [P39](./P39/src/lib.rs) (*) A list of prime numbers.

Given a range of integers by its lower and upper limit, construct a list of all prime numbers in that range.

Example: [examples/list_primes.rs](./P39/examples/list_primes.rs)
```rust
println!("{:?}", list_primes_in_range(7, 31));
```

```bash
P39 $ cargo run -q --example list_primes
[7, 11, 13, 17, 19, 23, 29, 31]
```

### [P40](./P40/src/lib.rs) (**) Goldbach's conjecture.

Goldbach's conjecture says that every positive even number greater than 2 is the sum of two prime numbers. E.g. 28 = 5 + 23. It is one of the most famous facts in number theory that has not been proved to be correct in the general case. It has been numerically confirmed up to very large numbers. Write a function to find the two prime numbers that sum up to a given even integer. 

Example: [examples/goldbach.rs](./P40/examples/goldbach.rs)
```rust
let (p1, p2) = goldbach(28);
println!("{} = {} + {}", 28, p1, p2);

let (p1, p2) = goldbach(1_000_000);
println!("{} = {} + {}", 1_000_000, p1, p2);
```

```bash
P40 $ cargo run -q --example goldbach
28 = 5 + 23
1000000 = 17 + 999983
```

### [P41](./P41/src/lib.rs) (**) A list of Goldbach compositions.

Given a range of integers by its lower and upper limit, print a list of all even numbers and their Goldbach composition. 

Example: [examples/goldbach_list.rs](./P41/examples/goldbach_list.rs)
```rust
for (n, p1, p2) in goldbach_list(9, 20) {
    println!("{} = {} + {}", n, p1, p2);
}
```

```bash
P41 $ cargo run -q --example goldbach_list
10 = 3 + 7
12 = 5 + 7
14 = 3 + 11
16 = 3 + 13
18 = 5 + 13
20 = 3 + 17
```

In most cases, if an even number is written as the sum of two prime numbers, one of them is very small. Very rarely, the primes are both bigger than, say, 50. Try to find out how many such cases there are in the range 2..2000.

Example: [examples/goldbach_list_limited.rs](./P41/examples/goldbach_list_limited.rs)
```rust
for (n, p1, p2) in goldbach_list_limited(1, 2000, 50) {
    println!("{} = {} + {}", n, p1, p2);
}
```

```bash
P41 $ cargo run -q --example goldbach_list_limited
992 = 73 + 919
1382 = 61 + 1321
1856 = 67 + 1789
1928 = 61 + 1867
```