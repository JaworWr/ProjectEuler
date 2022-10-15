use crate::utils::{vec_2d_default, Sieve};

fn num_sums(sieve: &Sieve, x: usize) -> usize {
    // dp[i, j] = # of ways to write j with prime #i
    let mut dp = vec_2d_default(x + 1, x + 1);
    dp[0][0] = 1;
    for i in 1..=x {
        let p = sieve.primes[i - 1];
        for j in 0..=x {
            dp[i][j] = dp[i - 1][j];
            if j >= p {
                dp[i][j] += dp[i][j - p];
            }
        }
    }
    dp[x][x]
}

pub fn solve() {
    let sieve = Sieve::new(10000);
    println!("{:?}", &sieve.primes[..10]);
    println!("10: {}", num_sums(&sieve, 10));
    for i in 11..10000 {
        let res = num_sums(&sieve, i);
        if res > 5000 {
            println!("{}: {}", i, res);
            break;
        }
    }
}
