use std::borrow::Cow;
use std::iter::FromIterator;

#[derive(Debug, Clone, PartialEq)]

struct Country<'a> {
    name: Cow<'a, str>,
    cities: Vec<Cow<'a, str>>,
}

#[allow(dead_code)]
impl<'a> Country<'a> {
    fn new<S>(name: S) -> Country<'a>
    where
        S: Into<Cow<'a, str>>,
    {
        Country {
            name: name.into(),
            cities: vec![],
        }
    }
    fn add_city<S>(&mut self, city_name: S)
    where
        S: Into<Cow<'a, str>>,
    {
        self.cities.push(city_name.into());
    }
}

impl<'a> IntoIterator for Country<'a> {
    type Item = Cow<'a, str>;
    type IntoIter = ::std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.cities.to_vec().into_iter()
    }
}

impl<'a> FromIterator<Cow<'a,str>> for Country<'a> {
    fn from_iter<I>(iter : I) -> Self
    where I: IntoIterator<Item=Cow<'a, str>> {
        let mut data = Self::new("Restored");
        for elem in iter {
            data.add_city(elem);
        }

        data
    }
}

#[test]
fn from_iterator_test() {

    let mut country = Country::new("Spain");
    country.add_city("Madrid");
    country.add_city("Barcelona".to_string());
    country.add_city("Cadiz".to_string());

    let mut results = vec![];



    for city in country.clone().into_iter() {
        results.push(city);
    }

    assert_eq!(results, vec!["Madrid", "Barcelona", "Cadiz"]);

    let it = country.into_iter();
    let c = Country::from_iter(it);

    assert_eq!(c,
        Country{ name: Cow::Borrowed("Restored"),
        cities: vec![Cow::Borrowed("Madrid"), Cow::Borrowed("Barcelona"), Cow::Borrowed("Cadiz")]});

}
