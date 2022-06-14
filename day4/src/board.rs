use std::convert::From;
use std::fmt::Display;

const BOARD_SIZE: usize = 5;

#[derive(Debug, Clone, PartialEq)]
pub struct Board {
    numbers: Vec<i32>,
    has_won: bool,
}

impl Board {
    pub fn mark_position(&mut self, pos: usize) {
        self.numbers[pos] = -1;
    }

    pub fn calculate_winning_score(&self, winning_num: i32) -> i32 {
        self.numbers
            .iter()
            .filter(|&i| *i != -1)
            .map(|&i| i)
            .sum::<i32>()
            * winning_num
    }

    pub fn is_winner(&mut self) -> bool {
        // check rows
        let mut start = 0;
        for _ in 0..BOARD_SIZE {
            if self.numbers[start]
                + self.numbers[start + 1]
                + self.numbers[start + 2]
                + self.numbers[start + 3]
                + self.numbers[start + 4]
                == -5
            {
                return true;
            }
            start += 5
        }
        start = 0;
        for _ in 0..BOARD_SIZE {
            if self.numbers[start]
                + self.numbers[start + 5]
                + self.numbers[start + 10]
                + self.numbers[start + 15]
                + self.numbers[start + 20]
                == -5
            {
                return true;
            }
            start += 1
        }
        // check cols
        return false;
    }

    // checks if number exists in bingo board and returns the index of the value
    pub fn has_num(&self, num: i32) -> Option<usize> {
        for i in 0..self.numbers.len() {
            if self.numbers[i] == num {
                return Some(i);
            }
        }
        None
    }
}

impl From<&str> for Board {
    fn from(s: &str) -> Self {
        Self {
            numbers: s
                .split('\n')
                .map(|s| s.trim().split(' ').collect::<Vec<&str>>())
                .flatten()
                .filter(|&s| s != "")
                .map(|s| s.parse::<i32>().unwrap())
                .collect(),
            has_won: false,
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buffer = String::new();
        for (i, num) in self.numbers.clone().iter().enumerate() {
            if i == 0 {
                buffer.push('|');
            }
            if i % 5 == 0 && i != 0 {
                buffer.push_str("|\n|");
            }
            buffer.push_str(&format!("{:3} ", *num))
        }
        buffer.push_str("|");
        write!(f, "{}", buffer)
    }
}
