use crate::utils::{Sieve, isqrt};
use crate::benchmark::{benchmark};
use std::collections::HashSet;

const MAX: usize = 100000000;

fn triples(sieve: &Sieve) -> HashSet<(usize, usize, usize)> {
    let (m, _) = isqrt(MAX);
    let mut res = HashSet::new();
    for a in 1..=MAX {
        for l in 2..=m {
            if a * l * l > MAX {
                break;
            }
            for k in 1..l {
                let p1 = a * k * k - 1;
                let p2 = a * k * l - 1;
                let p3 = a * l * l - 1;
                if sieve.is_prime[p1] && sieve.is_prime[p2] && sieve.is_prime[p3] {
                    res.insert((p1, p2, p3));
                }
            }
        }
    }
    res
}

pub fn solve() {
    let sieve = benchmark!(Sieve::new(MAX));
    let triples = benchmark!(triples(&sieve));
    // println!("{:?}", triples);
    println!("{}", triples.into_iter().map(|(a, b, c)| a+b+c).sum::<usize>());
}
