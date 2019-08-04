use std::ops::{Add, Neg};
use std::cmp::{PartialEq};
///Struct representing a Complex number
#[derive(Debug)]
pub struct Complex<T> {
    pub re: T,
    pub im: T
}

impl<T> PartialEq for Complex<T>
    where T: PartialEq {
        fn eq(&self, other: &Complex<T> ) -> bool {
            self.re == other.re && self.im == other.im
        }
    }

impl<T> Add for Complex<T>
    where T: Add<Output=T> {
        type Output = Self;
        fn add(self, rhs: Self) -> Self {
            Complex {
                re: self.re + rhs.re,
                im: self.im + rhs.im
            }
        }
    }

impl<T,O> Neg for Complex<T>
    where T: Neg<Output=O> {
        type Output = Complex<O>;
        fn neg(self) -> Complex<O> {
            Complex{ re: -self.re, im: -self.im}
        }
    }

#[test]
fn complex_add_test() {
    let complex1 = Complex{re: 2, im: 5};
    let complex2 = Complex{ re: 5, im: 3};

    assert_eq!(complex1 + complex2, Complex{ re: 7, im: 8});
}

#[test]
fn complex_negate_test() {

    let complex1 = Complex{ re: 5, im: 10};
    let complex2 = &(-complex1);

   assert_eq!(complex2.re, -5);
   assert_eq!(complex2.im, -10);

}