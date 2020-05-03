use serde::{Deserialize, Serialize};

use crate::canvas::line::Line;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Path(Vec<Line>);

impl Path {
    pub fn new(lines: Vec<Line>) -> Self {
        Path(lines)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, step: usize) -> Option<&Line> {
        self.0.get(step)
    }

    pub fn lines(&self) -> &Vec<Line> {
        &self.0
    }
}
