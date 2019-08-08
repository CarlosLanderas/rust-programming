#[test]
fn nth_consumer_test() {
    let mut squares = (0..10).map(|i| i*i);
    assert_eq!(squares.nth(4), Some(16));
    assert_eq!(squares.nth(0), Some(25));
    assert_eq!(squares.nth(6), None);
}

#[test]
fn nth_consumer_test_2() {
    let mut chars = "Oh my god!".chars();

    assert_eq!(chars.nth(3), Some('m'));
    assert_eq!(chars.nth(3), Some('o'));
    assert_eq!(chars.nth(0), Some('d'));
    assert_eq!(chars.nth(4), None);
}