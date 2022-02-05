use std::collections::HashSet;

pub fn solve() {
    let max = 10u128.pow(12);
    let upper = (max as f64).sqrt() as u128 + 10;
    let mut found = HashSet::new();
    found.insert(1);
    for b in 2u128..upper {
        let mut prod = b.pow(3);
        let mut x = (prod - 1) / (b - 1);
        while x < max {
            found.insert(x);
            prod *= b;
            x = (prod - 1) / (b - 1);
        }
    }
    
    println!("{}", found.iter().sum::<u128>());
}
