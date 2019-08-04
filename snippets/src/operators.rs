use std::ops::Add;
use std::cmp::{PartialEq,PartialOrd, Ord, Ordering, Eq};

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

impl PartialOrd for Money {

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }

}

impl PartialEq for Money {
     fn eq(&self, other: &Self) -> bool {
         self.value == other.value
     }
}


impl Ord for Money {
    fn cmp(&self, other: &Self) -> Ordering {
           match self.value {
            v if v == other.value => Ordering::Equal,
            v if v >= other.value => Ordering::Greater,
            v if v <= other.value => Ordering::Less,
            _ => panic!("Invalid value")
        }
    }
}


#[test]
fn money_add_test() {

    let money1 = Money{symbol : "€".to_string(), value : 10.5};
    let money2 = Money{symbol: "€".to_string(), value: 9.0};

    assert_eq!(money1 + money2, Money{ symbol: "€".to_string(), value: 19.5});

    let money3 = Money{symbol : "€".to_string(), value : 20.5};
    let money4 = Money{symbol: "€".to_string(), value: 10.0};
    let mut moneyvec = vec![money3, money4];
    moneyvec.sort();

    assert_eq!(moneyvec,
            vec![Money{symbol: "€".to_string(), value: 10.0},
            Money{symbol : "€".to_string(), value : 20.5}]);
}