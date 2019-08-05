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

fn count_selected_cities<F>(cities: &Vec<City>, test_fn: F) -> usize
where
    F: Fn(&City) -> bool,
{
    let mut count = 0;
    for city in cities {
        if test_fn(city) {
            count += 1;
        }
    }
    count
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
    );
}

#[test]
fn closures_test() {
    let cities = vec![
        City::new("Vergen", 450000, "Stonk"),
        City::new("Slujhan", 600000, "Tiranthia"),
    ];

    let limit = 500000;
    let total_cities = count_selected_cities(&cities, |city| city.population > limit);
    assert_eq!(total_cities, 1);
}
