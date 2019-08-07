use std::vec::Vec;
#[test]
fn chain_test() {
    let s = vec!["Hello", "Mr", "Snow"];

    let l = s.iter().filter(|w| !w.starts_with("S"));
    let r = s.iter().skip(2);

    let chained: Vec<&str> = l.chain(r).cloned().collect();

    assert_eq!(chained, s);
}
