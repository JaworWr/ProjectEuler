use crate::benchmark::benchmark;
use crate::utils::is_prime;

const MAX: usize = 10usize.pow(12);
const MASK: usize = (1<<32) - 1;

fn hamming_numbers() -> Vec<usize> {
    let mut res = Vec::new();
    let mut i = 1;
    while i <= MAX {
        let mut j = 1;
        while i * j <= MAX {
            let mut k = 1;
            while i * j * k <= MAX {
                res.push(i * j * k);
                k *= 5;
            }
           j *= 3;
        }
        i *= 2;
    }
    res.sort();
    res
}

fn add(a: usize, b: usize) -> usize {
    ((a & MASK) + (b & MASK)) & MASK 
}

fn result(h: &[usize], hp: &[usize], cur: usize) -> usize {
    if hp.is_empty() {
        let mut res = 0;
        for &x in h {
            let (cur1, overflow) = cur.overflowing_mul(x);
            if !overflow && cur1 <= MAX {
                res = add(res, cur1);
            }
            else {
                break;
            }
        }
        res
    }
    else {
        let mut res = result(h, &hp[1..], cur);
        let (cur1, overflow) = cur.overflowing_mul(hp[0] + 1);
        if !overflow && cur1 <= MAX {
            res = add(res, result(h, &hp[1..], cur1));
        }
        res
    }
}

pub fn solve() {
    let h = benchmark!(hamming_numbers());
    eprintln!("{} Hamming numbers", h.len());
    eprintln!("{:?}", &h[..10]);
    let hamming_primes = h[4..].iter()
        .copied()
        .filter(|&x| is_prime(x + 1));
    let hp: Vec<_> = benchmark!(hamming_primes.collect());
    eprintln!("{} Hamming primes", hp.len());
    eprintln!("{:?}", &hp[..10]);
    let s = benchmark!(result(&h, &hp, 1));
    println!("{}", s);
}
