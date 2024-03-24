use crate::utils::{euclid, Factors};

const M: i64 = 1500000;

pub fn solve() {
    let mut n_primitive = [0; M as usize].to_vec();
    let mut n_total = [0; M as usize].to_vec();

    for m in 1..=M {
        let ms = if m % 2 == 1 { m + 2 } else { m + 1 };
        for s in (ms..2*m).step_by(2) {
            assert!(s % 2 == 1);
            let y = 2 * m * s;
            if y > M {
                break;
            }
            let (gcd, _, _) = euclid(m, s);
            if gcd != 1 {
                continue;
            }
            n_primitive[y as usize - 1] += 1;
        }
    }
    
    for i in 2..=M {
        for j in Factors::of(i) {
            n_total[i as usize - 1] += n_primitive[j as usize - 1];
        }
    }

    println!("{:?}", &n_primitive[0..50]);
    println!("{:?}", &n_total[0..50]);
    println!("{}", n_primitive[120 - 1]);
    println!("{}", n_total[120 - 1]);
    
    let solution = n_total.iter().filter(|&&x| x == 1).count();
    println!("{}", solution);
}
