use std::cmp::{Ordering, PartialEq, PartialOrd};
use std::ops::{Add, Neg};
///Struct representing a Complex number
#[derive(Debug)]
pub struct Complex<T> {
    pub re: T,
    pub im: T,
}

impl<T> PartialEq for Complex<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Complex<T>) -> bool {
        self.re == other.re && self.im == other.im
    }
}

impl<T> Add for Complex<T>
where
    T: Add<Output = T>,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl<T, O> Neg for Complex<T>
where
    T: Neg<Output = O>,
{
    type Output = Complex<O>;
    fn neg(self) -> Complex<O> {
        Complex {
            re: -self.re,
            im: -self.im,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Interval<T> {
    lower: T,
    upper: T,
}

impl<T> PartialOrd<Interval<T>> for Interval<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Interval<T>) -> Option<Ordering> {
        match self {
            s if s == other => Some(Ordering::Equal),
            s if s.lower >= other.upper => Some(Ordering::Greater),
            s if s.upper <= other.lower => Some(Ordering::Less),
            _ => None,
        }
    }
}

// fn partial_cmp(&self, other: &Rhs) -> Option<Ordering>;

#[test]
fn complex_add_test() {
    let complex1 = Complex { re: 2, im: 5 };
    let complex2 = Complex { re: 5, im: 3 };

    assert_eq!(complex1 + complex2, Complex { re: 7, im: 8 });
}

#[test]
fn complex_negate_test() {
    let complex1 = Complex { re: 5, im: 10 };
    let complex2 = &(-complex1);

    assert_eq!(complex2.re, -5);
    assert_eq!(complex2.im, -10);
}

#[test]
fn interval_ordering_test() {
    assert!(Interval{ lower: 10, upper:20} < Interval{lower: 20, upper:40});
    assert!(Interval{ lower: 7, upper: 8} >= Interval{lower: 0, upper: 1});
    assert!(Interval{ lower: 7, upper: 8} <= Interval{lower: 7, upper: 8});
}