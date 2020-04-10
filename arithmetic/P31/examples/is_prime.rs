use P31::Primes;

pub fn main() {
    let mut primes = Primes::new();
    println!("Is {} a prime number? : {}", 53, primes.is_prime(53));
    println!("Is {} a prime number? : {}", 1957, primes.is_prime(1957));
}
