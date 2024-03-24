use std::ops::ShrAssign;

use num_traits::{PrimInt, NumAssign, Signed};

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

pub struct LowFactors<T> {
    i: T,
    x: T,
}

impl<T: PrimInt> LowFactors<T> {
    pub fn of(x: T) -> Self {
        Self {
            i: T::one(),
            x,
        }
    }
}

impl<T> Iterator for LowFactors<T> 
where T: PrimInt + NumAssign
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        while self.i.pow(2) <= self.x {
            let i = self.i;
            self.i += T::one();
            if self.x % i == T::zero() {
                return Some(i);
            }
        }
        None
    }
}

pub struct Factors<T> {
    low: LowFactors<T>,
    snd: Option<T>,
}

impl<T: PrimInt> Factors<T> {
    pub fn of(x: T) -> Self {
        Self {
            low: LowFactors::of(x),
            snd: None,
        }
    }
}

impl<T> Iterator for Factors<T>
where 
    T: PrimInt,
    LowFactors<T>: Iterator<Item=T>,
{
    type Item = T;

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

pub fn euclid<T: PrimInt + Signed>(mut a: T, mut b: T) -> (T, T, T) {
    let mut x = T::from(1).unwrap();
    let mut y = T::from(0).unwrap();
    let mut r = T::from(0).unwrap();
    let mut s = T::from(1).unwrap();
    
    while b != T::zero() {
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

pub fn isqrt<T: PrimInt>(x: T) -> (T, bool) {
    let f = x
        .to_f64()
        .unwrap()
        .sqrt()
        .floor();
    let s = T::from(f).unwrap();
    (s, s * s == x)
}

pub fn is_prime<T: PrimInt + NumAssign>(x: T) -> bool {
    if x < T::from(2).unwrap() {
        return false;
    }
    let mut i = T::from(2).unwrap();
    while i * i <= x {
        if x % i == T::zero() {
            return false;
        }
        i += T::one();
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

pub fn to_digits<T: PrimInt + NumAssign>(mut x: T, radix: T) -> Vec<i8> {
    assert!(x > T::zero());
    let mut res = Vec::new();
    while x > T::zero() {
        res.push((x % radix).to_i8().unwrap());
        x /= radix;
    }
    res.reverse();
    res
}

pub fn from_digits<T: PrimInt>(digits: &[i8], radix: T) -> T {
    let mut res = T::zero();
    for &d in digits {
        let d = T::from(d).unwrap();
        res = res * radix + d;
    }
    res
}

pub fn pow_mod<T: PrimInt + NumAssign, U: PrimInt + ShrAssign>(mut a: T, mut b: U, m: T) -> T {
    // calculate a^b mod m
    if b < U::zero() {
        return T::zero();
    }
    a %= m;
    let mut res = T::one();
    while b > U::zero() {
        if b & U::one() == U::one() {
            res = res * a % m;
        }
        b >>= U::one();
        a = a * a % m;
    }
    res
}

const MR_BASES: [u128; 7] = [2, 325, 9375, 28178, 450775, 9780504, 1795265022];

pub fn miller_rabin(n: u64) -> bool {
    // Miller-Rabin deterministic primality test - returns true iff n is prime
    // Time: O(log^2 n * log log n)
    if n < 2 {
        return false;
    }
    let n = n as u128;
    let mut s = 0;
    let mut d = n - 1;
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }
    'witness_loop: for a in MR_BASES {
        if a + 2 > n {
            break 'witness_loop;
        }
        let mut x = pow_mod(a, d, n);
        if x == 1 || x == n - 1 {
            continue 'witness_loop;
        }
        for _ in 1..s {
            x = x * x % n;
            if x == n - 1 {
                continue 'witness_loop;
            }
        }
        return false;
    }
    true
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
    fn isqrt() {
        let n: u64 = 1234567;
        assert_eq!(super::isqrt(n * n), (1234567, true));
        assert_eq!(super::isqrt(n * n + 1), (1234567, false));
        assert_eq!(super::isqrt(n * n - 1), (1234566, false));
    }

    #[test]
    fn is_prime() {
        assert_eq!(super::is_prime(1), false);
        assert_eq!(super::is_prime(2), true);
        assert_eq!(super::is_prime(23), true);
        assert_eq!(super::is_prime(24), false);
        assert_eq!(super::is_prime(25), false);
        assert_eq!(super::is_prime(-23), false);
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

    #[test]
    fn test_pow() {
        for a in [5u64, 17, 384] {
            for b in [4, 7] {
                for m in [13, 52] {
                    assert_eq!(pow_mod(a, b, m), a.pow(b) % m, "a = {}, b = {}, m = {}", a, b, m);
                }
            }
        }
    }

    #[test]
    fn test_miller_rabin() {
        let primes = [2, 3, 5, 7, 189068656106720591, 7377423892827990697];
        let non_primes = [0, 1, 4, 6, 1122004669633, 12731271997462];
        for p in primes {
            assert!(miller_rabin(p), "{} is prime but miller_rabin returned false", p);
        }
        for q in non_primes {
            assert!(!miller_rabin(q), "{} is non-prime but miller_rabin returned true", q);
        }
    }
}
