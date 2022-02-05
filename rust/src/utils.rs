pub struct Sieve {
    pub max: usize,
    pub is_prime: Vec<bool>,
    pub spf: Vec<usize>,
    pub primes: Vec<usize>,
}

impl Sieve {
    pub fn new(max: usize) -> Self {
        let mut is_prime = Vec::with_capacity(max + 1);
        let mut spf = Vec::with_capacity(max + 1);
        let mut primes = Vec::new();

        is_prime.resize(max + 1, true);
        is_prime[0] = false;
        is_prime[1] = false;
        spf.resize(max + 1, 0);

        for i in 2..=max {
            if is_prime[i] {
                primes.push(i);
                spf[i] = i;
            }
            
            for &p in &primes {
                if i * p > max || p > spf[i] {
                    break;
                }
                is_prime[i * p] = false;
                spf[i * p] = p;
            }
        }

        Self { max, is_prime, spf, primes }
    }

    pub fn prime_factors(&self, x: usize) -> PrimeFactors {
        PrimeFactors { x, spf: &self.spf }
    }

    pub fn totient(&self, mut x: usize) -> usize {
        let mut spf_last = 1;
        let mut result = x;
        while x > 1 {
            if self.spf_ext(x) != spf_last {
                result = result / self.spf_ext(x) * (self.spf_ext(x) - 1);
                spf_last = self.spf_ext(x);
            }
            x /= self.spf_ext(x);
        }
        result
    }

    pub fn spf_ext(&self, x: usize) -> usize {
        // works up to max^2
        // O(1) for x < max
        // O(sqrt(x) / log(sqrt(x))) for x >= max
        if x < self.max {
            self.spf[x]
        }
        else {
            for &p in &self.primes {
                if x % p == 0 {
                    return p;
                }
                if p * p > x {
                    break;
                } 
            }
            x
        }
    }

    pub fn is_prime_ext(&self, x: usize) -> bool {
        self.spf_ext(x) == x
    }
}

pub struct PrimeFactors<'a> {
    x: usize,
    spf: &'a [usize],
}

impl<'a> Iterator for PrimeFactors<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.x > 1 {
            let spf = self.spf[self.x];
            self.x /= spf;
            Some(spf)
        }
        else {
            None
        }
    }
}

pub struct LowFactors {
    i: usize,
    x: usize,
}

impl LowFactors {
    pub fn of(x: usize) -> Self {
        Self {
            i: 1,
            x,
        }
    }
}

impl Iterator for LowFactors {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        while self.i.pow(2) <= self.x {
            let i = self.i;
            self.i += 1;
            if self.x % i == 0 {
                return Some(i);
            }
        }
        None
    }
}

pub struct Factors {
    low: LowFactors,
    snd: Option<usize>,
}

impl Factors {
    pub fn of(x: usize) -> Self {
        Self {
            low: LowFactors::of(x),
            snd: None,
        }
    }
}

impl Iterator for Factors {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.snd.is_some() {
            return self.snd.take();
        }
        let fst = self.low.next()?;
        let snd = self.low.x / fst;
        if fst != snd {
            self.snd = Some(snd);
        }
        Some(fst)
    }
}

pub fn euclid(mut a: i64, mut b: i64) -> (i64, i64, i64) {
    let mut x = 1;
    let mut y = 0;
    let mut r = 0;
    let mut s = 1;
    
    while b != 0 {
        let q = a / b;
        let c = a % b;
        a = b;
        b = c;

        let x1 = r;
        let y1 = s;
        r = x - q * r;
        s = y - q * s;
        x = x1;
        y = y1;
    }
    (a, x, y)
}

pub fn neg_mod(a: i64, b: i64) -> i64 {
    if a >= 0 {
        a % b
    }
    else {
        b - (-a % b)
    }
}

pub fn isqrt(x: usize) -> (usize, bool) {
    let s = (x as f64).sqrt().floor() as usize;
    (s, s * s == x)
}

pub fn is_prime(x: usize) -> bool {
    if x < 2 {
        return false;
    }
    let mut i = 2;
    while i * i <= x {
        if x % i == 0 {
            return false;
        }
        i += 1;
    }
    true
}

pub fn vec_2d_with<T, F: FnMut() -> T>(n_rows: usize, n_cols: usize, mut fill_fn: F) -> Vec<Vec<T>> {
    let mut res = Vec::with_capacity(n_rows);
    res.resize_with(n_rows, || {
        let mut row = Vec::with_capacity(n_cols);
        row.resize_with(n_cols, &mut fill_fn);
        row
    });
    res
}

pub fn vec_2d_default<T: Default>(n_rows: usize, n_cols: usize) -> Vec<Vec<T>> {
    vec_2d_with(n_rows, n_cols, Default::default)
}

pub fn vec_2d<T: Clone>(n_rows: usize, n_cols: usize, fill_value: T) -> Vec<Vec<T>> {
    vec_2d_with(n_rows, n_cols, || fill_value.clone())
}

fn to_digits(mut x: usize, radix: usize) -> Vec<i8> {
    let mut res = Vec::new();
    while x > 0 {
        res.push((x % radix) as i8);
        x /= radix;
    }
    res.reverse();
    res
}

fn from_digits(digits: &[i8], radix: usize) -> usize {
    let mut res = 0;
    for &d in digits {
        let d = d as usize;
        res = res * radix + d;
    }
    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sieve() {
        let max = 30;
        let sieve = Sieve::new(max);
        let is_prime = vec![
            false, 
            false, true, true, false, true, false, true, false, false, false,
            true, false, true, false, false, false, true, false, true, false,
            false, false, true, false, false, false, false, false, true, false,        
        ];
        let spf = vec![
            0,
            0, 2, 3, 2, 5, 2, 7, 2, 3, 2,
            11, 2, 13, 2, 3, 2, 17, 2, 19, 2,
            3, 2, 23, 2, 5, 2, 3, 2, 29, 2,
        ];
        let primes = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
        for i in 0..=max {
            assert_eq!(sieve.is_prime[i], is_prime[i], "incorrect is_prime, number: {}", i);
            assert_eq!(sieve.spf[i], spf[i], "incorrect spf, number: {}", i);
        }
        assert_eq!(sieve.primes, primes);
    }

    #[test]
    fn totient() {
        let sieve = Sieve::new(1000);
        assert_eq!(sieve.totient(1), 1);
        assert_eq!(sieve.totient(5), 4);
        assert_eq!(sieve.totient(9), 6);
        assert_eq!(sieve.totient(12), 4);
        assert_eq!(sieve.totient(2345), 1584);
    }

    #[test]
    fn prime_factors() {
        let sieve = Sieve::new(1000);
        assert_eq!(sieve.prime_factors(1).collect::<Vec<_>>(), vec![]);
        assert_eq!(sieve.prime_factors(19).collect::<Vec<_>>(), vec![19]);
        assert_eq!(sieve.prime_factors(60).collect::<Vec<_>>(), vec![2, 2, 3, 5]);
        assert_eq!(sieve.prime_factors(324).collect::<Vec<_>>(), vec![2, 2, 3, 3, 3, 3]);
        assert_eq!(sieve.prime_factors(450).collect::<Vec<_>>(), vec![2, 3, 3, 5, 5]);        
    }


    #[test]
    fn euclid() {
        assert_eq!(super::euclid(174, 18), (6, -1, 10));
        assert_eq!(super::euclid(18, 174), (6, 10, -1));
        assert_eq!(super::euclid(13, 5), (1, 2, -5));
    }

    #[test]
    fn low_factors() {
        assert_eq!(LowFactors::of(1).collect::<Vec<_>>(), vec![1]);
        assert_eq!(LowFactors::of(7).collect::<Vec<_>>(), vec![1]);
        assert_eq!(LowFactors::of(36).collect::<Vec<_>>(), vec![1, 2, 3, 4, 6]);
    }

    #[test]
    fn factors() {
        assert_eq!(Factors::of(1).collect::<Vec<_>>(), vec![1]);
        assert_eq!(Factors::of(7).collect::<Vec<_>>(), vec![1, 7]);
        assert_eq!(Factors::of(36).collect::<Vec<_>>(), vec![1, 36, 2, 18, 3, 12, 4, 9, 6]);
    }

    #[test]
    fn neg_mod() {
        assert_eq!(super::neg_mod(14, 6), 2);
        assert_eq!(super::neg_mod(-14, 6), 4);
    }

    #[test]
    fn isqrt() {
        assert_eq!(super::isqrt(12345 * 12345), (12345, true));
        assert_eq!(super::isqrt(12345 * 12345 + 1), (12345, false));
        assert_eq!(super::isqrt(12345 * 12345 - 1), (12344, false));
    }

    #[test]
    fn is_prime() {
        assert_eq!(super::is_prime(1), false);
        assert_eq!(super::is_prime(2), true);
        assert_eq!(super::is_prime(23), true);
        assert_eq!(super::is_prime(24), false);
        assert_eq!(super::is_prime(25), false);
    }

    #[test]
    fn to_digits() {
        assert_eq!(super::to_digits(123, 10), vec![1, 2, 3]);
        assert_eq!(super::to_digits(0b1010, 2), vec![1, 0, 1, 0]);
    }

    #[test]
    fn from_digits() {
        assert_eq!(super::from_digits(&[1, 2, 3], 10), 123);
        assert_eq!(super::from_digits(&[1, 0, 1, 0], 2), 0b1010);
    }
}
