use crate::utils::Sieve;

const MOD: usize = 500500507;

// let x = p1^a1 * p2^a2 * ...
// number of factors of x = (a1 + 1)(a2 + 1)...
// hence, each ai must be of the form 2^k - 1 = sum_i=0^(k-1) 2^i
// we peek the 500500 smallest factors of the form p^2^k
// max for sieve and max factor chosen by hand
pub fn solve() {
    let sieve = Sieve::new(10000000);
    let mut candidates: Vec<usize> = Vec::new();
    for km in 1..=20 {
        let k = 1 << km;
        candidates.extend(
            sieve.primes.iter().filter_map(|&x| {
                if (x as f64) < (usize::MAX as f64).powf(1. / k as f64) {
                    Some(x.pow(k))
                } else {
                    None
                }
            })
        );
    }
    candidates.extend(sieve.primes.into_iter());
    candidates.sort();

    const N: usize = 500500;
    let mut res = 1;
    for &x in &candidates[..N] {
        res = (res * x) % MOD;
    }
    println!("{}", res);
}
