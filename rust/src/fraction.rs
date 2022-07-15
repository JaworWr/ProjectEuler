use std::ops::{Add, Sub, Mul, Div, Neg};
use std::cmp::Ordering;
use std::fmt;

use crate::utils::euclid;

#[derive(Debug, Clone, Copy)]
pub struct Fraction {
    num: i64,
    denom: i64,
}

impl Fraction {
    // INVARIANT: denom > 0
    pub fn new(mut num: i64, mut denom: i64) -> Option<Fraction> {
        if denom < 0 {
            denom = -denom;
            num = -num;
        }
        if denom == 0 {
            None
        }
        else {
            Some(Fraction {
                num: num,
                denom: denom,
            })
        }
    }

    pub fn num(self) -> i64 {
        self.num
    }

    pub fn denom(self) -> i64 {
        self.denom
    }

    pub fn inv(self) -> Option<Fraction> {
        Fraction::new(self.denom, self.num)
    }

    pub fn reduce_by(self, mut d: i64) -> Fraction {
        assert_ne!(d, 0);
        if d < 0 { 
            d = -d;
        }
        debug_assert!(self.num % d == 0, "Numerator must be divisible by {}", d);
        debug_assert!(self.denom % d == 0, "Denominator must be divisible by {}", d);
        Fraction {
            num: self.num / d,
            denom: self.denom / d,
        }
    }

    pub fn reduce(self) -> Fraction {
        let (d, _, _) = euclid(self.num, self.denom);
        self.reduce_by(d)
    }

    pub fn is_reduced(self) -> bool {
        let (d, _, _) = euclid(self.num, self.denom);
        d == 1
    }

    pub fn as_f64(self) -> f64 {
        self.num as f64 / self.denom as f64
    }
}

impl From<i64> for Fraction {
    fn from(x: i64) -> Self {
        Fraction {
            num: x,
            denom: 1,
        }
    }
}

impl Neg for Fraction {
    type Output = Fraction;

    fn neg(self) -> Self::Output {
        Fraction {
            num: -self.num,
            denom: self.denom,
        }
    }
}

impl Add for Fraction {
    type Output = Fraction;

    fn add(self, rhs: Fraction) -> Self::Output {
        Fraction {
            num: self.num * rhs.denom + self.denom * rhs.num,
            denom: self.denom * rhs.denom,
        }
    }
}

impl Sub for Fraction {
    type Output = Fraction;

    fn sub(self, rhs: Fraction) -> Self::Output {
        self + (-rhs)
    }
}

impl Mul for Fraction {
    type Output = Fraction;

    fn mul(self, rhs: Self) -> Self::Output {
        Fraction {
            num: self.num * rhs.num,
            denom: self.denom * rhs.denom,
        }
    }
}

impl Div for Fraction {
    type Output = Fraction;

    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.inv().expect("Denominator must not be 0")
    }
}

impl PartialEq for Fraction {
    fn eq(&self, other: &Self) -> bool {
        self.num * other.denom == self.denom * other.num
    }
}

impl Eq for Fraction {}

impl PartialOrd for Fraction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Fraction {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.num * other.denom).cmp(&(self.denom * other.num))
    }
}

impl fmt::Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.num, self.denom)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn neg() {
        let a = Fraction { num: 3, denom: 5 };
        assert_eq!(-a, Fraction { num: -3, denom: 5 });
        assert_eq!(a.inv().unwrap(), Fraction { num: 5, denom: 3 });
        assert_eq!((-a).inv().unwrap(), Fraction { num: -5, denom: 3 });
    }

    #[test]
    fn arithmetic() {
        let a = Fraction { num: 3, denom: 5 };
        let b = Fraction { num: 2, denom: 3 };
        assert_eq!(a + b, Fraction { num: 19, denom: 15 });
        assert_eq!(a - b, Fraction { num: -1, denom: 15 });
        assert_eq!(a * b, Fraction { num: 6, denom: 15 });
        assert_eq!(a / b, Fraction { num: 9, denom: 10});
    }

    #[test]
    fn reduce() {
        let a = Fraction { num: 18, denom: 24 };
        assert_eq!(a.reduce_by(3), Fraction { num: 6, denom: 8 });
        assert_eq!(a.reduce(), Fraction { num: 3, denom: 4 });
    }

    #[test]
    fn eq() {
        let a = Fraction { num: 2, denom: 3 };
        let b = Fraction { num: 6, denom: 9 };
        let c = Fraction { num: 7, denom: 9 };
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn cmp() {
        let a = Fraction { num: 4, denom: 6 };
        let b = Fraction { num: 7, denom: 9 };
        assert!(a < b);
        assert!((-a) > (-b));
    }
}
