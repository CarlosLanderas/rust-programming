use chrono::prelude::*;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(Hash, Debug, PartialEq)]
struct MuseumNumber {
    value: u32,
}

#[derive(Debug)]
struct Artifact {
    id: MuseumNumber,
    name: String,
    date: DateTime<Utc>,
}

impl PartialEq for Artifact {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Artifact {}

impl Hash for Artifact {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.id.hash(hasher);
    }
}

#[test]
fn hash_trait_test() {
    let mut col = HashSet::new();
    col.insert(Artifact {
        id: MuseumNumber { value: 32 },
        name: String::from("Indian bow"),
        date: Utc::now(),
    });

    let mut col2 = HashSet::new();
    col2.insert(Artifact {
        id: MuseumNumber { value: 32 },
        name: String::from("Indian bow"),
        date: Utc::now(),
    });

    assert_eq!(col, col2);
}
