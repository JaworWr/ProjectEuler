use crate::fraction::Fraction;

// max numerator n such that n/d < f
fn max_num(f: Fraction, d: i64) -> i64 {
    let mut a = 0;
    let mut b = d;
    while b - a > 1 {
        let s = (a + b) / 2;
        let f1 = Fraction::new(s, d).unwrap();
        if f1 < f {
            a = s;
        } else {
            b = s;
        }
    }
    a
}

// min numerator n such that n/d > f
fn min_num(f: Fraction, d: i64) -> i64 {
    let mut a = 0;
    let mut b = d;
    while b - a > 1 {
        let s = (a + b) / 2;
        let f1 = Fraction::new(s, d).unwrap();
        if f1 <= f {
            a = s;
        } else {
            b = s;
        }
    }
    b
}

fn count_fractions(f1: Fraction, f2: Fraction, d: i64) -> usize {
    let a = min_num(f1, d);
    let b = max_num(f2, d);
    (a..=b)
        .map(|n| Fraction::new(n, d).unwrap())
        .filter(|f| f.is_reduced())
        .count()
}

pub fn solve() {
    let f1 = Fraction::new(1, 3).unwrap();
    let f2 = Fraction::new(1, 2).unwrap();
    let ans = (4..=12000)
        .map(|d| count_fractions(f1, f2, d))
        .sum::<usize>();
    println!("{}", ans);
}
