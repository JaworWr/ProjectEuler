use crate::utils::Sieve;
use cached::proc_macro::cached;
use lazy_static::lazy_static;

const N1: usize = 100000;
const N2: usize = 10000;

lazy_static!{
    static ref SIEVE: Sieve = Sieve::new(N1);
}

fn n_digits(n: usize) -> u32 {
    (n as f64 + 1.).log10().ceil() as u32
}

fn concatenate(n: usize, m: usize) -> usize {
    n * 10usize.pow(n_digits(m)) + m
}

#[cached]
fn check( a: usize, b: usize) -> bool {
    SIEVE.is_prime_ext(concatenate(a, b)) && SIEVE.is_prime_ext(concatenate(b, a))
}

fn check_all(a: impl IntoIterator<Item = usize>, b: usize) -> bool {
    a.into_iter().all(|p| check(p, b))
}

// basically a heavily optimized brute force
pub fn solve() {
    let mut best_sum = usize::MAX;
    let mut best = Vec::new();

    let primes: Vec<_> = SIEVE.primes
        .iter()
        .copied()
        .take_while(|&p| p < N2)
        .enumerate()
        .collect();

    for &(i1, p1) in &primes {
        if p1 >= best_sum {
            break;
        }
        for &(i2, p2) in &primes[i1+1..] {
            if p1 + p2 >= best_sum {
                break;
            }
            if !check(p1, p2) {
                continue;
            }
            for &(i3, p3) in &primes[i2+1..] {
                let s = p1 + p2 + p3;
                if best_sum <= s {
                    break;
                }
                if !check_all([p1, p2], p3) {
                    continue;
                }
                for &(i4, p4) in &primes[i3+1..] {
                    let s = p1 + p2 + p3 + p4;
                    if best_sum <= s {
                        break;
                    }
                    if !check_all([p1, p2, p3], p4) {
                        continue;
                    }
                    
                    for &(_, p5) in &primes[i4+1..] {
                        let s = p1 + p2 + p3 + p4 + p5;
                        if best_sum <= s {
                            break;
                        }
                        if !check_all([p1, p2, p3, p4], p5) {
                            continue;
                        }
                        best_sum = s;
                        best = vec![p1, p2, p3, p4, p5];
                    }
                }
            }
        }
    }
    println!("{:?}, {}", best, best_sum);
}
