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

fn belongs_to_country(country: String) -> impl Fn(&City) -> bool {
    move |city| city.country.to_lowercase() == country.to_lowercase()
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
    let end_with_cities = count_selected_cities(&cities, |city| city.name.ends_with("n"));

    let country = String::from("stonk");

    let belongs_func = belongs_to_country(country);
    let cities_belong_to = count_selected_cities(&cities, belongs_func);

    assert_eq!(total_cities, 1);
    assert_eq!(end_with_cities, 2);
    assert_eq!(cities_belong_to, 1);
}
