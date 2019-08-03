
pub struct Cell {
    x: f64,
    y: f64,
}

impl Cell {
    pub fn distance_from_origin(&self) -> f64 {
        f64::hypot(self.x, self.y)
    }
}