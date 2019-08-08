#[test]
fn by_ref_test() {
    use std::collections::HashMap;

    let text = "Hello\r\nMister\r\nDeeJay";

    let mut lines = text.lines();

    //by_ref avoids the loop taking ownership of lines
    for line in lines.by_ref() {
        println!("{}", line);
    }

    for _ in lines {
        assert!(true);
    }
}
