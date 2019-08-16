use std::str::FromStr;
use traits::Wallet;
fn main() {
    let wallet = Wallet::from_str("234.5;0x123422");
    match wallet {
        Ok(r) => println!("{:?}", r),
        Err(e) => println!("{:?}", e),
    };
}
