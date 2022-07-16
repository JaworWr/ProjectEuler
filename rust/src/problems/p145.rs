use crate::benchmark::Progress;

fn is_odd_digit(c: char) -> bool {
    c.to_digit(10)
        .map(|x| x % 2 == 1)
        .unwrap_or(false)
}

fn check(x: u32) -> bool {
    if x % 10 == 0 {
        return false;
    }
    let xr: u32 = x.to_string()
        .chars()
        .rev()
        .collect::<String>()
        .parse()
        .unwrap();
    let r = x + xr;
    r.to_string()
        .chars()
        .all(is_odd_digit)
}

pub fn solve() {
    let ans = (1..=1000000000)
        .log_progress(10000000, "answer")
        .filter(|&x| check(x))
        .count();
    println!("{}", ans);
}
