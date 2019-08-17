use std::str::FromStr;
use std::convert::Into;

use traits::Wallet;
fn main() {

    let wallet = Wallet::from_str("234.54;0x1sf23ef5632sd");

    match wallet {
        Ok(r) => println!("{:?}", r),
        Err(e) => println!("{:?}", e),
    };

    let wallet2 = Wallet::from_str("325.23;0x1sf2sdxw12").unwrap();
    let amount : f64 = wallet2.into();
    println!("Amount from wallet is {}", amount);

}
