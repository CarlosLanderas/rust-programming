#[test]
fn iterators_filter_map_test() {
    use std::str::FromStr;

    let mut results = Vec::new();
    let text = "aloha 3.14 notanumber 2t 6.18 15";
    for number in text
        .split_whitespace()
        .filter_map(|w| f64::from_str(w).ok())
    {
        results.push(number);
    }
    assert_eq!(results, [3.14, 6.18, 15.0]);
}

#[test]
fn interators_flat_map_test() {
    use std::collections::HashMap;

    let mut tv_shows = HashMap::new();

    tv_shows.insert("suits", vec!["Harvy", "Mike", "Donna"]);
    tv_shows.insert("dragon ball", vec!["Goku", "Gohan", "Freezer"]);
    tv_shows.insert("got", vec!["John Snow", "Daenerys", "Tyrion"]);

    let shows = ["suits", "got"];
    let mut results = vec![];

    for &show in shows.iter().flat_map(|sh| &tv_shows[sh]) {
        &results.push(show);
    }

    assert_eq!(
        results,
        ["Harvy", "Mike", "Donna", "John Snow", "Daenerys", "Tyrion"]
    )
}
