use std::fmt::Display;

use crate::point::Point;

#[derive(Clone)]
pub struct Line {
    start: Point,
    end: Point,
}

impl Line {
    pub fn new(_start: Point, _end: Point) -> Self {
        Self {
            start: _start,
            end: _end,
        }
    }

    pub fn start(&self) -> Point {
        self.start.clone()
    }

    pub fn end(&self) -> Point {
        self.end.clone()
    }

    pub fn points(&self) -> [Point; 2] {
        [self.start.clone(), self.end.clone()]
    }

    pub fn is_horizontal(&self) -> bool {
        self.start.y() == self.end.y()
    }

    pub fn is_vertical(&self) -> bool {
        self.start.x() == self.end.x()
    }

    pub fn is_diagonal(&self) -> bool {
        (self.start.x() - self.end.x()).abs() == (self.start.y() - self.end.y()).abs()
    }
}

impl From<String> for Line {
    fn from(s: String) -> Self {
        let arr: Vec<&str> = s.split(" -> ").collect();
        Self {
            start: Point::from(arr[0]),
            end: Point::from(arr[1]),
        }
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Line: {} -> {}", self.start, self.end)
    }
}
