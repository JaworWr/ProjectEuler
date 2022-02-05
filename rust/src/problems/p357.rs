use crate::utils::{Sieve, LowFactors};
use crate::benchmark::benchmark;

fn interesting(sieve: &Sieve, x: usize) -> bool {
    LowFactors::of(x)
        .all(|d| sieve.is_prime[d + x / d])
}

pub fn solve() {
    let max = 100000000;
    let sieve = benchmark!(Sieve::new(max + 10));
    let nums = (1..=max).filter(|&x| interesting(&sieve, x));
    let sum = benchmark!(nums.sum::<usize>());
    println!("{}", sum);
}
