use crate::utils::Sieve;

fn check_prime(sieve: &Sieve, p: usize) -> bool {
    let mut p1 = p;
    while p1 > 0 {
        if !sieve.is_prime[p1] {
            return false;
        }
        p1 /= 10;
    }
    
    let mut d = 10;
    while d < p {
        if !sieve.is_prime[p % d] {
            return false;
        }
        d *= 10;
    }
    true
}

pub fn solve() {
    let sieve = Sieve::new(10000000);

    let primes = &sieve.primes[4..];
    let mut truncatable = Vec::new();

    for &p in primes {
        if check_prime(&sieve, p) {
            truncatable.push(p);
        }
        if truncatable.len() >= 11 {
            break;
        }
    }
    assert_eq!(truncatable.len(), 11);
    println!("{}", truncatable.iter().sum::<usize>());
}
