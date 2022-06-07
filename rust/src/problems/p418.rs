// a bruteforce-ish solutions with quite a few heuristics
// main idea - a, b, c should all be close to the cube root of n
use std::collections::HashMap;
use itertools::Itertools;
use ordered_float::NotNan;
use crate::utils::Sieve;
use crate::benchmark::{benchmark, Progress};

// for p1^i1 * p2^i2 * ... * pn^in stores i1, i2, ..., in
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct FactorCounts(Vec<u8>);

impl FactorCounts {
    fn empty(size: usize) -> Self {
        FactorCounts(vec![0; size])
    }

    fn set_factor(&self, idx: usize, n: u8) -> Self {
        let mut res = self.clone();
        res.0[idx] = n;
        res
    }

    fn subtract(&self, rhs: &Self) -> Self {
        let res_vec = self.0.iter().copied()
            .zip(rhs.0.iter().copied())
            .map(|(x, y)| x - y)
            .collect();
        FactorCounts(res_vec)
    }

    fn get_log(&self, logs: &[NotNan<f64>]) -> NotNan<f64> {
        self.0.iter()
            .zip(logs)
            .map(|(&i, &l)| l * NotNan::new(i as f64).unwrap())
            .sum()
    }

    fn get_value(&self, prime_factors: &[usize]) -> u64 {
        self.0.iter()
            .zip(prime_factors)
            .map(|(&i, &f)| (f as u64).pow(i as u32))
            .product()
    }
}

// get factors as prime counts and as their logs for comparison
fn get_factors(
        prime_factor_counts: &FactorCounts, 
        prime_factor_logs: &[NotNan<f64>],
        idx: usize,
        max_log: NotNan<f64>,
        cur_factor: FactorCounts,
        cur_log: NotNan<f64>) -> Vec<(FactorCounts, NotNan<f64>)> 
{
    if idx == prime_factor_counts.0.len() {
        return vec![(cur_factor, cur_log)];
    }

    let mut res = Vec::new();
    for i in 0..=prime_factor_counts.0[idx] {
        let i_f64 = NotNan::new(i as f64).unwrap();
        let new_log = cur_log + i_f64 * prime_factor_logs[idx];
        if new_log > max_log + 1e-6 {
            break;
        }
        let new_factor = cur_factor.set_factor(idx, i);
        let new_factors = get_factors(
            prime_factor_counts, 
            prime_factor_logs, 
            idx + 1, 
            max_log, 
            new_factor,
            new_log,
        );
        res.extend(new_factors);
    }
    res
}

pub fn solve() {
    const N: usize = 43;
    let sieve = Sieve::new(100);
    let mut prime_factors = HashMap::new();
    for i in 2..=N {
        for p in sieve.prime_factors(i) {
            prime_factors.entry(p)
                .and_modify(|x| *x += 1)
                .or_insert(1u8);
        }
    }
    let prime_factors = prime_factors.into_iter()
        .sorted()
        .collect_vec();
    eprintln!("{:?}", prime_factors);

    let prime_factor_logs = prime_factors.iter()
        .map(|x| NotNan::new((x.0 as f64).ln()).unwrap())
        .collect_vec();
    eprintln!("{:?}", prime_factor_logs);
    let total_log = (2..=N)
        .map(|x| NotNan::new((x as f64).ln()).unwrap())
        .sum::<NotNan<f64>>();
    eprintln!("{:?}", total_log.into_inner());

    let prime_factor_values = prime_factors.iter()
        .map(|x| x.0)
        .collect_vec();
    let prime_factor_counts = prime_factors.iter()
        .map(|x| x.1)
        .collect_vec();
    let prime_factor_counts = FactorCounts(prime_factor_counts);
    eprintln!("{:?}\n{:?}", prime_factor_values, prime_factor_counts);

    // get factors that are at most slightly larger than the cube root of n
    let mut factors = benchmark!(
        get_factors(&prime_factor_counts, &prime_factor_logs, 0, total_log * 0.35, 
            FactorCounts::empty(prime_factors.len()), NotNan::new(0.).unwrap())
    );
    eprintln!("{}", factors.len());
    benchmark!(factors.sort_by_key(|x| x.1));

    // initial a = the largest factor <= cube root of n
    let initial_a = match factors.binary_search_by_key(&(total_log / 3.), |x| x.1) {
        Ok(pos) => pos,
        Err(pos) => pos - 1,
    };
    eprintln!("{} {:?} {} {}", initial_a, factors[initial_a], factors[initial_a].0.get_value(&prime_factor_values), 
        (total_log / 3.).into_inner());

    // store the best triple and the log of the best ratio
    let mut best_d = NotNan::new(99999.0f64).unwrap();
    let mut best_triple = None;
    for i in (initial_a - 10000 ..= initial_a).rev().log_progress(1000, "a") {
        let (a, a_log) = &factors[i];
        for (b, b_log) in &factors[i..] {
            let c = prime_factor_counts
                .subtract(&a)
                .subtract(&b);
            let c_log = c.get_log(&prime_factor_logs);
            if c_log < b_log - 1e-6 {
                break;
            }
            let d = c_log - a_log;
            if d < best_d {
                best_d = d;
                best_triple = Some((a, b, c));
            }
        }
    }

    eprintln!("{:?}", best_triple);
    let (a, b, c) = best_triple.unwrap();
    eprintln!("{:?} {:?} {:?}", a.get_log(&prime_factor_logs), b.get_log(&prime_factor_logs), c.get_log(&prime_factor_logs));
    let a = a.get_value(&prime_factor_values);
    let b = b.get_value(&prime_factor_values);
    let c = c.get_value(&prime_factor_values);
    println!("{} {} {} {}", a, b, c, a + b + c);
}