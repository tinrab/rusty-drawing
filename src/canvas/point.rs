use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Point(f64, f64);

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Point(x, y)
    }

    pub fn x(&self) -> f64 {
        self.0
    }

    pub fn y(&self) -> f64 {
        self.1
    }
}
