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

pub fn solve() {
    let f = Fraction::new(3, 7).unwrap();
    let ans = (1..=1000000)
        .map(|d| Fraction::new(max_num(f, d), d).unwrap())
        .filter(|f| f.num() != 0 && f.is_reduced())
        .max()
        .unwrap();
    println!("{}", ans);
}
