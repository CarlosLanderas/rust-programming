#[test]
fn any_consumer_test() {

    let cities = vec!["Madrid", "Berlin", "Amsterdam", "China"];

    assert_eq!(cities.iter().any(|city| city.len() == 5), true);
    assert_eq!(cities.iter().any(|city| city.starts_with("Chi")), true);
    assert_eq!(cities.iter().any(|city| city.ends_with("Wai")), false);
}

#[derive(PartialEq)]
enum Family {
    Lannister,
    Stark,
    Targaryen
}
#[test]
fn all_consumer_test() {
    use std::collections::HashMap;

    let mut heroes = HashMap::new();
    heroes.insert("Tyrion", Family::Lannister);
    heroes.insert("Snow", Family::Stark);
    heroes.insert("Bran", Family::Stark);
    heroes.insert("Daenerys", Family::Targaryen);

    let all_stark = heroes.iter()
    .by_ref()
    .filter(|(k, _)| k.starts_with("S") || k.starts_with("B"))
    .all(|(_,v)| {
        match v {
            Family::Stark => true,
            _  => false
        }
    });

    assert_eq!(all_stark, true);

}