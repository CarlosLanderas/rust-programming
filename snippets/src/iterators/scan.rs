#[test]

fn scan_test() {
    let iter = (0..10).scan(0, |sum, item| {
        *sum += item;
        if *sum > 10 {
            None
        } else {
            Some(item * item)
        }
    });

    assert_eq!(iter.collect::<Vec<i32>>(), vec![0, 1, 4, 9, 16]);
}
