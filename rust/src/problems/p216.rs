use crate::utils::is_prime;
use crate::benchmark::Progress;

pub fn solve() {
    let m = 50000000usize;
    let c = (2..=m)
        .map(|x| 2 * x * x - 1)
        .log_progress(5000, "count")
        .filter(|x| is_prime(*x))
        .count();
    println!("{}", c);
}