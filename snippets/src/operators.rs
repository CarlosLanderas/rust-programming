use std::ops::Add;
use std::cmp::PartialEq;

#[derive(Debug)]
pub struct Money {
    pub symbol: String,
    pub value: f64
}

impl Add for Money {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            symbol: self.symbol,
            value: self.value + other.value
        }
    }
}

impl PartialEq for Money {
     fn eq(&self, other: &Self) -> bool {
         self.value == other.value
     }
}

#[test]
fn money_add_test() {

    let money1 = Money{symbol : "€".to_string(), value : 10.5};
    let money2 = Money{symbol: "€".to_string(), value: 9.0};

    assert_eq!(money1 + money2, Money{ symbol: "€".to_string(), value: 19.5});
}