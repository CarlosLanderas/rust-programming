use std::error::Error;
use std::fmt::{self, Display};
use std::str::FromStr;

#[derive(Debug)]
pub struct Wallet {
    amount: f64,
    address: String,
}

#[derive(Debug)]
pub struct ParseWalletFromString {
    error: String,
}

impl Display for ParseWalletFromString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.error)
    }
}

impl Into<f64> for Wallet {
    fn into(self)  -> f64 {
        self.amount
    }
}

impl std::error::Error for ParseWalletFromString {}

impl FromStr for Wallet {
    type Err = ParseWalletFromString;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split(";").collect::<Vec<_>>();
        let mut parts_iter = split.iter();

        if let Some(amount) = parts_iter.next() {
            if let Some(address) = parts_iter.next() {
                if let Ok(v) = amount.parse::<f64>() {
                    return Ok(Wallet {
                        amount: v,
                        address: address.to_string(),
                    });
                }
            }
        }
        Err(ParseWalletFromString {
            error: "Could not parse".to_string(),
        })
    }
}
