use crate::utils::Sieve;
use crate::benchmark::benchmark;

const MAX: usize = 100000000;

pub fn solve() {
    let sieve = benchmark!(Sieve::new(MAX));
    let mut primes_below = vec![0; MAX];
    for i in 1..MAX {
        primes_below[i] = primes_below[i - 1] + sieve.is_prime[i] as i64;
    }
    
    let mut total = 0;
    for p in sieve.primes {
        let lim = (MAX / p).min(p);
        total += primes_below[lim];
    }
    println!("{}", total);
}