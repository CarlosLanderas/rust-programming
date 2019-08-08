#[test]
fn iterator_sum_consumer_test() {
    let s: u32 = (1..20 + 1).sum();
    assert_eq!(s, 210);
}

#[test]
fn iterator_factorial_consumer_test() {
    let s: u32 = (1..5 + 1).product();
    assert_eq!(s, 120);
}

#[test]
fn max_min_consumer_test() {
    assert_eq!([0, 30, 12, 200, 67, 40].iter().max(), Some(&200));
    assert_eq!([10, 30, 12, 200, 67, 40].iter().min(), Some(&10));
}

#[test]
fn max_min_by_cmp_consumer_test() {
    let cmp = |l: &&f32, r: &&f32| -> std::cmp::Ordering { l.partial_cmp(r).unwrap() };
    let numbers = [2.0, 4.0, 1.0, 6.0];

    let max = &numbers.iter().max_by(cmp);
    let min = &numbers.iter().min_by(cmp);

    assert_eq!(*max, Some(&6.0));
    assert_eq!(*min, Some(&1.0));
}
