use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::fmt;
use std::ops::Add;

#[derive(Debug)]
pub struct Money {
    pub symbol: String,
    pub value: f64,
}

impl Add for Money {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            symbol: self.symbol,
            value: self.value + other.value,
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
            _ => panic!("Invalid value"),
        }
    }
}

impl Eq for Money {}

#[derive(PartialEq)]
pub struct User {
    pub name: String,
    pub age: usize,
}

impl Add for User {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            name: self.name.add(" ").add(&other.name),
            age: self.age + other.age,
        }
    }
}

impl fmt::Debug for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Name: {}, Age: {}", self.name, self.age)
    }
}

#[test]
fn money_add_test() {
    let money1 = Money {
        symbol: "€".to_string(),
        value: 10.5,
    };
    let money2 = Money {
        symbol: "€".to_string(),
        value: 9.0,
    };

    assert_eq!(
        money1 + money2,
        Money {
            symbol: "€".to_string(),
            value: 19.5
        }
    );

    let money3 = Money {
        symbol: "€".to_string(),
        value: 20.5,
    };
    let money4 = Money {
        symbol: "€".to_string(),
        value: 10.0,
    };
    let mut moneyvec = vec![money3, money4];
    moneyvec.sort();

    assert_eq!(
        moneyvec,
        vec![
            Money {
                symbol: "€".to_string(),
                value: 10.0
            },
            Money {
                symbol: "€".to_string(),
                value: 20.5
            }
        ]
    );
}

#[test]
fn user_add_test() {
    let user1 = User {
        name: "Carlos".to_string(),
        age: 34,
    };
    let user2 = User {
        name: "Pedro".to_string(),
        age: 20,
    };

    let user_sum = user1 + user2;
    assert_eq!(user_sum.name, "Carlos Pedro".to_string());
    assert_eq!(user_sum.age, 54);
}

#[test]
fn user_eq_test() {
    let user3 = User {
        name: "Pedro".to_string(),
        age: 20,
    };
    let user4 = User {
        name: "Pedro".to_string(),
        age: 20,
    };

    //This works because User implements Debug trait
    assert_eq!(user3, user4);
}
