use crate::utils::Sieve;
use itertools::Itertools;

const N: usize = 1000000;

pub fn solve() {
    let sieve = Sieve::new(1000005);
    let prefix_sums = sieve.primes.iter()
        .scan(0, |s, &x| {
            let res = Some(*s);
            *s += x;
            res
        })
        .take_while(|&s| s < N)
        .collect_vec();
    
    let mut best_len = 1;
    let mut best_val = 0;
    for i in 1..prefix_sums.len() {
        for j in 0..i - best_len {
            let s = prefix_sums[i] - prefix_sums[j];
            if sieve.is_prime[s] {
                let l = i - j;
                if l > best_len {
                    best_len = l;
                    best_val = s;
                }
            }
        }
    }
    println!("{} {}", best_len, best_val);
}