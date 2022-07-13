// use std::hash::Hash;
use crate::benchmark::benchmark;
use cached::proc_macro::cached;
use cached::UnboundCache;
use itertools::Itertools;

#[derive(Debug, Clone)]
struct Val {
    value: u64,
    k: u8,
    l: u8
}

impl Val {
    fn new(k: u8, l: u8) -> Self {
        let value = 2u64.pow(k as u32) * 3u64.pow(l as u32);
        Self { value, k, l }
    }

    fn immediate_factors(&self) -> Vec<Val> {
        let mut res = Vec::new();
        if self.k > 0 {
            res.push(Val::new(self.k - 1, self.l));
        }
        if self.l > 0 {
            res.push(Val::new(self.k, self.l - 1))
        }
        res
    }
}

impl PartialEq for Val {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for Val {}

// impl Hash for Val {
//     fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
//         self.value.hash(state);
//     }
// }

impl PartialOrd for Val {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl Ord for Val {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}

struct ValData {
    values: Vec<Val>,
    idx: Vec<Vec<usize>>,
}

impl ValData {
    fn generate(max: u64) -> Self {
        let mut values = Vec::new();
        let mut idx = Vec::new();
        for k in 0..64 {
            for l in 0..41 {
                let val = Val::new(k, l);
                if val.value > max {
                    break;
                }
                values.push(val);
            }
        }
        values.sort();
        
        for (i, val) in values.iter().enumerate() {
            if val.l == 0 {
                idx.push(vec![i]);
                assert_eq!(idx.len(), val.k as usize + 1);
            }
            else {
                idx[val.k as usize].push(i);
                assert_eq!(idx[val.k as usize].len(), val.l as usize + 1);
            }
        }
        Self { values, idx }
    }

    fn get_value_index(&self, value: &Val) -> Option<usize> {
        self.idx.get(value.k as usize)
            .and_then(|v| v.get(value.l as usize))
            .map(|&x| x)
    }

    fn check_start(&self, starts: &[usize], new_start: Val) -> Option<usize> {
        for &x in starts {
            if new_start.value % self.values[x].value == 0 {
                return None
            }
        }
        self.get_value_index(&new_start)
    }

    fn ways(&self) -> f64 {
        ways_with_starts(self, &[0])
    }
}


#[cached(
    type = "UnboundCache<Vec<usize>, f64>",
    create = "{ UnboundCache::new() }",
    convert = "{ starts.to_owned() }",
)]
fn ways_with_starts(vd: &ValData, starts: &[usize]) -> f64 {
    if starts.len() == 0 {
        return 1.;
    }

    let mut res = 0.;
    for (i, &x) in starts.iter().enumerate() {
        let start = &vd.values[x];
        let mut new_starts = starts[..i].to_owned();
        new_starts.extend(&starts[i + 1..]);
        
        let new_start1 = vd.check_start(&new_starts, Val::new(start.k + 1, start.l));
        new_starts.extend(new_start1);
        let new_start2 = vd.check_start(&new_starts, Val::new(start.k, start.l + 1));
        new_starts.extend(new_start2);
        new_starts.sort();
        new_starts.dedup();

        res += ways_with_starts(vd, &new_starts);
    }
    res
}

pub fn solve() {
    // const N: u64 = 8;
    const N: u64 = 10u64.pow(4);
    // const N: u64 = 10u64.pow(18);

    let vd = benchmark!(ValData::generate(N));
    eprintln!("{}", vd.values.len());
    let first_values = vd.values.iter()
        .map(|x| x.value)
        .take(10)
        .collect_vec();
    eprintln!("{:?}", first_values);
    println!("{:.10e}", benchmark!(vd.ways()));
}
