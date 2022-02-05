use crate::utils::Sieve;
use crate::benchmark::benchmark;

fn to_digits(mut x: usize, radix: usize) -> Vec<i8> {
    let mut res = Vec::new();
    while x > 0 {
        res.push((x % radix) as i8);
        x /= radix;
    }
    res.reverse();
    res
}

fn from_digits(digits: &[i8], radix: usize) -> usize {
    let mut res = 0;
    for &d in digits {
        let d = d as usize;
        res = res * radix + d;
    }
    res
}

fn replace_digits(base: &[i8], mask: usize) -> Vec<usize> {
    let mut mask_digits = to_digits(mask, 2);
    let diff = base.len() - mask_digits.len();
    if diff > 0 {
        let mut padded = vec![0; diff];
        padded.append(&mut mask_digits);
        mask_digits = padded;
    }
    let base_digits: Vec<i8> = base.iter()
        .zip(&mask_digits)
        .map(|(&d, &m)| d * (1 - m))
        .collect();
    let base = from_digits(&base_digits, 10);
    let mask = from_digits(&mask_digits, 10);

    let start = if mask_digits[0] == 0 {
        0
    } else {
        1
    };
    (start..=9).map(|x| base + x * mask).collect()
}

fn check_prime(sieve: &Sieve, prime: usize) -> (usize, usize) {
    let digits = to_digits(prime, 10);
    let max_mask = 1 << digits.len();
    let mut best = 0;
    let mut best_mask = 0;
    for m in 1..max_mask {
        let candidates = replace_digits(&digits, m);
        let score = candidates.iter()
            .filter(|&&x| sieve.is_prime[x])
            .count();
        if best < score {
            best = score;
            best_mask = m;
        }
    }
    (best, best_mask)
}

pub fn solve() {
    let sieve = Sieve::new(10000000);
    let res= benchmark!(check_prime(&sieve, 13));
    println!("{:?}", res);
    let res = benchmark!(check_prime(&sieve, 56123));
    println!("{:?}", res);

    let iter = sieve.primes.iter()
        .skip_while(|&&p| p < 56003);
    for &p in iter {
        let (score, mask) = check_prime(&sieve, p);
        if score == 8 {
            println!("{} {:b}", p, mask);
            let digits = to_digits(p, 10);
            let results = replace_digits(&digits, mask);
            println!("{:?}", results);
            let min_prime = results.into_iter()
                .filter(|&p| sieve.is_prime[p])
                .next()
                .unwrap();
            println!("{}", min_prime);
            break;
        }
    }
}
