use crate::benchmark::Progress;

fn get_stations(n: i64) -> Vec<i64> {
    if n == 1 {
        return vec![0];
    }

    let mut res = Vec::new();
    let mut x = 1;
    let mut y = 1;
    for _i in 0..=2 * n {
        res.push((x, y));
        x = x * 2 % n;
        y = y * 3 % n;
    }
    res.sort();
    res.dedup();
    res.into_iter()
        .map(|p| p.1)
        .collect()
}

fn lis(seq: &Vec<i64>) -> i64 {
    // add index to each element so that they are unique
    let seq_unique = seq.iter()
        .enumerate()
        .map(|(i, &x)| (x, i + 1));
    
    // d[i] = the smallest element, at which a sequence of length i can terminate
    let mut d = vec![(i64::MAX, usize::MAX); seq.len() + 1];
    let mut l = 0;
    d[0] = (0, 0);

    for x in seq_unique {
        let pos = d.binary_search(&x)
            .expect_err("duplicate stations");
        assert!(pos <= seq.len());
        d[pos] = x;
        l = l.max(pos as i64);
    }
    l
}

fn calc_s(n: i64) -> i64 {
    let s = get_stations(n);
    lis(&s)
}

pub fn solve() {
    eprintln!("{}", calc_s(1));
    eprintln!("{}", calc_s(2));
    eprintln!("{}", calc_s(22));
    eprintln!("{}", calc_s(123));
    eprintln!("{}", calc_s(10000));
    let res: i64 = (1..=30)
        .map(|k: i64| k.pow(5))
        .map(calc_s)
        .log_progress(1, "solution")
        .sum();
    println!("{}", res);
}
