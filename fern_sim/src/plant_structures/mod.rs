
pub mod roots;
pub mod stems;
pub mod leaves;

pub use self::leaves::Leaf;
pub use self::roots::Root;

use self::roots::RootSet;
use self::stems::StemSet;

pub enum FernType {
    FiddleHead
}

pub struct Fern {
    pub roots: RootSet,
    pub stems: StemSet
}

impl Fern {
    pub fn new(_type: FernType) -> Fern {
        Fern{
            roots: vec![],
            stems: vec![stems::Stem{furled: true}]
        }
    }

    pub fn is_furled(&self) -> bool { !self.is_fully_unfurled()}

    pub fn is_fully_unfurled(&self) -> bool {
        self.stems.iter().all(|s| !s.furled)
    }
}