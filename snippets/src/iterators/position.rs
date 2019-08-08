#[test]
fn position_consumer_test() {

    let text = "Show me the code";

    let m_position = text.chars().position(|c| c == 'm');

    let z = 'z';

    let z_position = text.chars().position(|c| c == z);

    assert_eq!(m_position, Some(5));
    assert_eq!(z_position, None);

}