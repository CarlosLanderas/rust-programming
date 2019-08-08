#[test]
fn position_consumer_test() {
    let text = "Show me the code";

    let m_position = text.chars().position(|c| c == 'm');

    let z = 'z';

    let z_position = text.chars().position(|c| c == z);

    assert_eq!(m_position, Some(5));
    assert_eq!(z_position, None);
}

#[test]
fn r_position_consumer_test() {
    let text = "Are you a boy scout?";

    assert_eq!(text.as_bytes().iter().rposition(|&c| c == b'b'), Some(10));
    assert_eq!(text.as_bytes().iter().rposition(|&c| c == b'A'), Some(0));
}
