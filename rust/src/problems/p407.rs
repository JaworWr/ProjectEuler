use crate::utils::{Sieve, euclid, neg_mod};
use crate::benchmark::{benchmark, Progress};
use itertools::Itertools;

const MAX: usize = 10000000;

// finds pairs k, l such that k * l == x and gcd(k, l) == 1
fn find_pairs(factors: &[usize], idx: usize) -> Vec<(usize, usize)> {
    if idx == factors.len() {
        vec![(1, 1)]
    }
    else {
        let v = find_pairs(factors, idx + 1);
        let mut res: Vec<_> = v.iter()
            .map(|&(x, y)| (factors[idx] * x, y))
            .collect();
        if idx > 0 {
            res.extend(v.into_iter().map(|(x, y)| (x, factors[idx] * y)));
        }
        res
    }
}

fn solve_x(sieve: &Sieve, x: usize) -> usize {
    let factors: Vec<_> = sieve
        .prime_factors(x)
        .dedup_with_count()
        .map(|(c, p)| p.pow(c as u32))
        .collect();
    
    find_pairs(&factors, 0)
        .into_iter()
        .skip(1)
        .map(|(a, b)| {
            let a = a as i64;
            let b = b as i64;
            let (d, k, l) = euclid(a, b);
            assert_eq!(d, 1);
            let k = neg_mod(k, b);
            let l = neg_mod(l, a);           
            (k * a).max(l * b) as usize
        })
        .max()
        .unwrap_or(1)
}

pub fn solve() {
    let sieve = benchmark!(Sieve::new(MAX));
    let result: usize = (2..=MAX)
        .map(|x| solve_x(&sieve, x))
        .log_progress(MAX / 20, "solutions")
        .sum();
    println!("{}", result);
}
