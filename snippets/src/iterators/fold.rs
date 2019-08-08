#[test]

fn fold_consumer_test() {
    let str_col = [
        "This ",
        "text ",
        "should ",
        "be ",
        "all ",
        "together ",
        ":/",
    ];
    let text = str_col.iter().fold(String::new(), |mut s, &w| {
        s.push_str(w);
        s
    });
    assert_eq!(text, "This text should be all together :/");
}

#[test]
fn fold_consumer_test_2() {
    let nums = [0, 2, 3, 4, 5];
    let inc_by = &2;

    let result = nums.iter().fold(0, |mut prev, curr| {
        prev += curr + inc_by;
        prev
    });
    assert_eq!(result, 24);
}
