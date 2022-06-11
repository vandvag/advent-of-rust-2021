use std::cmp::Eq;
use std::fmt::Display;

#[derive(Clone, Eq)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(_x: i32, _y: i32) -> Self {
        Self { x: _x, y: _y }
    }
    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    pub fn into_tuple(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}

impl From<&str> for Point {
    fn from(s: &str) -> Self {
        let nums: Vec<i32> = s.split(',').map(|x| x.parse::<i32>().unwrap()).collect();
        Self {
            x: nums[0],
            y: nums[1],
        }
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
