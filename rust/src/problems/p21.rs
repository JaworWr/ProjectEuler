use crate::utils::Factors;

fn d(x: usize) -> usize {
    Factors::of(x).sum::<usize>() - x
}

pub fn solve() {
    let mut cnt = 0;
    for i in 1..10000 {
        let j = d(i);
        if d(j) == i && i != j {
            cnt += i;
        }
    }
    println!("{}", cnt);
}