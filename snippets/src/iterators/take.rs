#[test]
fn take_test() {
    let message = "This is text\r\n\
                   with more content\r\n\
                   \r\n\
                   And now more content\r\n\
                   :";

    let mut results = vec![];
    for content in message.lines().take_while(|l| !l.is_empty()) {
        &results.push(content);
    }

    assert_eq!(results, vec!["This is text", "with more content"])
}
