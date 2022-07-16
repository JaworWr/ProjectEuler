use itertools::Itertools;

use crate::utils::Sieve;

pub fn solve() {
    const N: usize = 10000000;
    const K: usize = 4;
    let sieve = Sieve::new(N);

    let mut c = 0;
    let mut first = 0;
    for x in 2..N {
        let pf = sieve.prime_factors(x)
            .unique()
            .collect_vec();
        if pf.len() == K {
            if c == 0 {
                first = x;
            }
            c += 1;
        } else {
            c = 0;
        }
        if c == K {
            break;
        }
    }
    println!("{}", first);
}