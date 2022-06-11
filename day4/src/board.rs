use std::convert::From;
use std::fmt::Display;

const BOARD_SIZE: usize = 5;

#[derive(Debug, Clone, PartialEq)]
pub struct Board {
    numbers: Vec<u32>,
    marked: Vec<bool>,
    has_won: bool,
}

impl Board {
    pub fn board(&self) -> Vec<u32> {
        self.numbers.clone()
    }

    pub fn mark_position(&mut self, pos: usize) {
        self.marked[pos] = true;
    }

    pub fn marked(&self) -> Vec<bool> {
        self.marked.clone()
    }

    pub fn calculate_winning_score(&self, winning_num: u32) -> u32 {
        self.numbers
            .iter()
            .enumerate()
            .filter(|(i, _)| !self.marked[*i])
            .map(|(_, &val)| val)
            .sum::<u32>()
            * winning_num
    }

    pub fn is_winner(&mut self) -> bool {
        // check rows
        for chunk in self.marked().chunks(BOARD_SIZE) {
            if chunk.iter().all(|f| *f) {
                self.has_won = true;
                return true;
            }
        }

        // check cols
        for i in 0..BOARD_SIZE {
            if self
                .marked()
                .iter()
                .enumerate()
                .filter(|&(index, _)| index % BOARD_SIZE == i)
                .map(|(_, e)| *e)
                .all(|f| f)
            {
                self.has_won = true;
                return true;
            }
        }

        return false;
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
                .map(|s| s.parse::<u32>().unwrap())
                .collect(),
            marked: vec![false; (BOARD_SIZE * BOARD_SIZE) as usize],
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
            buffer.push_str(&format!("{} ", *num))
        }

        write!(f, "{}", buffer)
    }
}

pub struct BoardCollection {
    unfinished: Vec<Board>,
    completed: Vec<Board>,
}

impl BoardCollection {
    pub fn mark_boards(&mut self, number: u32) {
        for board in self.unfinished.iter_mut() {
            for (i, num) in board.board().iter().enumerate() {
                if *num == number {
                    board.mark_position(i);
                }
            }
        }
    }

    pub fn last_completed_board(&self) -> &Board {
        self.completed.last().unwrap()
    }

    pub fn pop_winning_board(&mut self, winning_board: Board) {
        // self.unfinished.retain(|x| *x != winning_board);
        self.completed.push(winning_board);
    }

    pub fn boards(&self) -> Vec<Board> {
        self.unfinished.clone()
    }

    pub fn all_completed(&self) -> bool {
        self.unfinished.len() == self.completed.len()
    }
}

impl From<Vec<Board>> for BoardCollection {
    fn from(bingo_boards: Vec<Board>) -> Self {
        Self {
            unfinished: bingo_boards,
            completed: Vec::new(),
        }
    }
}
