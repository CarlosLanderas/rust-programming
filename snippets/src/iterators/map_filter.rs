#[test]
fn iterators_map_test() {
    let text = "ponies \n giraffes\niguanas \nsquid".to_string();
    let v: Vec<&str> = text.lines().map(str::trim).collect();

    assert_eq!(v, ["ponies", "giraffes", "iguanas", "squid"]);
}

fn iterators_filter_test() {
    let text = "ponies \n giraffes\niguanas \nsquid".to_string();
    let v: Vec<&str> = text
        .lines()
        .map(str::trim)
        .filter(|s| *s == "iguanas")
        .collect();

    assert_eq!(v, ["ponies", "giraffes", "squid"]);
}
