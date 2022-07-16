use crate::utils::{Sieve, isqrt};

fn check(sieve: &Sieve, n: usize) -> bool {
    if sieve.is_prime[n] {
        return false;
    }

    let primes = sieve.primes.iter()
        .copied()
        .take_while(|&p| p < n);
    for p in primes {
        let s = (n - p) / 2;
        let (_, is_square) = isqrt(s);
        if is_square {
            return false;
        } 
    }
    true
}

pub fn solve() {
    const N: usize = 1000000;
    let sieve = Sieve::new(N);
    let ans = (3..N)
        .step_by(2)
        .filter(|&n| check(&sieve, n))
        .next()
        .unwrap_or(0);
    println!("{}", ans);
}
