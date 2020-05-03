use serde::{Deserialize, Serialize};

use crate::canvas::point::Point;

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Line(Point, Point);

impl Line {
    pub fn new(a: Point, b: Point) -> Self {
        Line(a, b)
    }

    pub fn a(&self) -> &Point {
        &self.0
    }

    pub fn b(&self) -> &Point {
        &self.1
    }
}
