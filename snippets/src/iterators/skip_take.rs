#[test]
fn take_while_test() {
    let message = "This is text\r\n\
                   with more content\r\n\
                   \r\n\
                   And now more content\r\n\
                   :";

    let mut results = vec![];
    for content in message.lines().take_while(|l| !l.is_empty()) {
        &results.push(content);
    }

    assert_eq!(results, vec!["This is text", "with more content"]);
}

#[test]
fn skip_test() {
    let message = "\r\n\
                   \r\n\
                   O\r\n\
                   M\r\n\
                   G";

    let mut results = vec![];

    for v in message.split_whitespace().skip_while(|l| l.is_empty()) {
        &results.push(v);
    }

    assert_eq!(results, vec!["O", "M", "G"]);
}
