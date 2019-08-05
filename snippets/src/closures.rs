#[derive(Debug, PartialEq)]
struct City<'a> {
    name: &'a str,
    population: i64,
    country: &'a str,
}

impl<'a> City<'a> {
    fn new(name: &'a str, population: i64, country: &'a str) -> City<'a> {
        City {
            name,
            population,
            country,
        }
    }
}

fn sort_cities(cities: &mut Vec<City>) {
    cities.sort_by_key(|city| -city.population)
}

#[test]
fn should_sort_by_key() {
    let city2 = City::new("Toro", 9000, "Spain");
    let city1 = City::new("Madrid", 7000000, "Spain");

    let mut cities = vec![city1, city2];
    sort_cities(&mut cities);

    assert_eq!(
        &cities,
        &vec![
            City::new("Madrid", 7000000, "Spain"),
            City::new("Toro", 9000, "Spain")
        ]
    )
}
