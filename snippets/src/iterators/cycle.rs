#[test]
fn cycle_test() {
    let names = ["Batman", "Superman", "Spiderman"];
    let mut cycle = names.iter().cycle();

    assert_eq!(cycle.next(), Some(&"Batman"));
    assert_eq!(cycle.next(), Some(&"Superman"));
    assert_eq!(cycle.next(), Some(&"Spiderman"));
    assert_eq!(cycle.next(), Some(&"Batman"));
    assert_eq!(cycle.next(), Some(&"Superman"));
}
