
#[derive(Debug, PartialEq)]
struct VideoGame {
    name: String,
    release_year : u32,
}

#[test]
fn find_consumer_test() {

    let video_games = vec![
        VideoGame{name : "Street Fighter".to_string(), release_year : 1987},
        VideoGame{name : "Tekken".to_string(), release_year : 1994},
        VideoGame{name : "World of warcraft".to_string(), release_year : 2004}
    ];

    let mut iterator = video_games.iter();

    let under_ninety = &iterator.find(|game| game.release_year < 1990);
    assert_eq!(*under_ninety, Some(&VideoGame{name : "Street Fighter".to_string(), release_year : 1987}));

    let first = &iterator.find(|game| game.release_year > 1990);
    let second = &iterator.find(|game| game.release_year > 2002);
    let third = &iterator.find(|game| game.release_year > 2010);

    assert_eq!(*first, Some(&VideoGame{name : "Tekken".to_string(), release_year: 1994}));
    assert_eq!(*second, Some(&VideoGame{name : "World of warcraft".to_string(), release_year: 2004}));
    assert_eq!(*third, None);

}