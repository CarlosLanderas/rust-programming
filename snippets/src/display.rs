
use std::fmt;

pub struct Complex
 {
    pub re: f64,
    pub im: f64,
}

impl fmt::Display for Complex
 {
    fn fmt(&self, dest: &mut fmt::Formatter) -> fmt::Result
     {

        let i_sign = if self.im < 0.0 { '-'} else {'+'};
        write!(dest, "{} {} {}i", self.re, i_sign, f64::abs(self.im))
    }
}

#[test]
fn display_trait_test() {
    let complex = Complex{re: -0.5, im: 0.866};

    assert_eq!(format!("{}", complex), "-0.5 + 0.866i");

    let complex2 = Complex{ re: 3.4, im: -0.444};

    assert_eq!(format!("{}", complex2), "3.4 - 0.444i");
}