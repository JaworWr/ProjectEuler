use crate::utils::vec_2d_default;
use crate::benchmark::benchmark;

fn num_sums(x: usize) -> usize {
    // dp[i, j] = # of ways to write j with highest number i
    let mut dp = vec_2d_default(x + 1, x + 1);
    dp[0][0] = 1;
    for i in 1..=x {
        for j in 0..=x {
            dp[i][j] = dp[i - 1][j];
            if j >= i {
                dp[i][j] += dp[i][j - i];
            }
        }
    }
    dp[x][x] - 1
}

pub fn solve() {
    println!("5: {}", num_sums(5));
    let res = benchmark!(num_sums(100));
    println!("100: {}", res);
}
