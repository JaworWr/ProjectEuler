use crate::benchmark::benchmark;
use crate::fraction::Fraction;
use crate::utils::Sieve;

const MAX: i64 = 1000000;

fn resilience(sieve: &Sieve, d: i64) -> Fraction {
    Fraction::new(
        sieve.totient(d as usize) as i64, 
        d - 1
    )
    .unwrap()
}

fn make_candidates(current: usize, max: usize, primes: &[usize]) -> Vec<i64> {
    if current > max || primes.is_empty() {
        return Vec::new();
    }
    let mut res = vec![current as i64];
    for (i, &p) in primes.iter().enumerate() {
        res.extend(make_candidates(p * current, max, &primes[i..]));
    }
    res
}

pub fn solve() {
    let sieve = benchmark!(Sieve::new(MAX as usize));
    println!("{}", resilience(&sieve, 12));

    // let m = Fraction::new(2, 5).unwrap();
    let m = Fraction::new(15499, 94744).unwrap().reduce();
    
    let mut num = 1;
    let mut denom = 1;
    let mut prev_denom = 1;
    let mut last_prime_idx = 0;
    let mut fraction = Fraction::from(1);

    for (i, &p) in sieve.primes.iter().enumerate() {
        prev_denom = denom;
        num *= (p - 1) as i64;
        denom *= p as i64;
        fraction = Fraction::new(num, denom - 1).unwrap();
        if fraction < m {
            last_prime_idx = i;
            break;
        }
    }
    println!("{} {} {} {}", prev_denom, denom, fraction, last_prime_idx);

    let mut candidates = make_candidates(1, denom as usize, 
        &sieve.primes[..last_prime_idx]);
    candidates.sort();
    println!("{}", candidates.len());
    
    for d in candidates {
        if d > prev_denom {
            let r = resilience(&sieve, d);
            if r < m {
                println!("{} {}", d, r);
                break;
            }
        }
    }
}
