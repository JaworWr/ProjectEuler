use std::mem;

// representation of the information fo the form (n0 zeros, n1 ones, ..., n9 nines)
// i.e. of a particular feasible combination of digit counts 
// since there are at most 3 of each digit, we can get away with just 2 beats per digit
// thus we need 20 bits, so we can fit in u32
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Ways(u32);

impl Ways {
    // how many digits n we have
    fn get_digit(self, n: u32) -> u32 {
        self.0 >> 2 * n & 3
    }

    // add another digit n, if possible
    fn add_digit(self, n: u32) -> Option<Self> {
        if self.get_digit(n) == 3 {
            None
        } else {
            Some(Ways(self.0 + (1 << 2 * n)))
        }
    }
}


const MAX_DIGITS: usize = 2usize.pow(20);


pub fn solve() {
    const N: usize = 18;
    // for each possible way to make a feasible number
    // we count how many numbers made this way we have
    // for each length i = 1, ..., N
    let mut way_counts = Vec::with_capacity(MAX_DIGITS);
    let mut tmp = Vec::with_capacity(MAX_DIGITS);
    way_counts.resize(MAX_DIGITS, 0u64);
    tmp.resize(MAX_DIGITS, 0u64);
    // intialize with signle digit numbers
    for d in 1..=9 {
        let w = Ways(0).add_digit(d).unwrap();
        way_counts[w.0 as usize] += 1;
    }
    for _i in 2..=N {
        for (j, &n) in way_counts.iter().enumerate() {
            // there are no numbers with this combination so far
            if n == 0 {
                continue;
            }
            let w = Ways(j as u32);
            for d in 0..=9 {
                if let Some(w1) = w.add_digit(d) {
                    tmp[w1.0 as usize] += n;
                }
            }
        }
        mem::swap(&mut way_counts, &mut tmp);
        tmp.fill(0);
    }
    println!("{}", way_counts.into_iter().sum::<u64>());
}
