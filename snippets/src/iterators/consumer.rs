#[test]
fn iterator_sum_consumer_test() {
    let s : u32 = (1..20+1).sum();
    assert_eq!(s, 210);
}

#[test]
fn iterator_factorial_consumer_test() {
    let s: u32 = (1..5 +1).product();
    assert_eq!(s, 120);
}

#[test]
fn max_min_consumer_test() {
    assert_eq!([0,30,12,200,67,40].iter().max(), Some(&200));
    assert_eq!([10,30,12,200,67,40].iter().min(), Some(&10));
}