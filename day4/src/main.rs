use std::fs;

mod board;
use board::Board;

fn main() {
    let filepath = "day4.in";
    let (nums_drawn, mut bingo_boards) = read_input(filepath);

    part1(&nums_drawn, &mut bingo_boards);
}

fn part1(nums_drawn: &Vec<i32>, bingo_boards: &mut Vec<Board>) {
    let mut found: bool = false;
    let mut index = 0;
    while !found && index < nums_drawn.len() {
        let number = nums_drawn[index];

        for board in bingo_boards.into_iter() {
            match board.has_num(number) {
                Some(pos) => {
                    board.mark_position(pos);
                }
                None => {}
            }
            if board.is_winner() {
                println!("BOARD: ");
                println!("{}", board);
                println!(
                    "has won with score: {}",
                    board.calculate_winning_score(number)
                );
                found = true;
            }
        }
        index += 1;
    }
}

fn read_input(filepath: &str) -> (Vec<i32>, Vec<Board>) {
    let data = match fs::read_to_string(filepath) {
        Ok(data) => data,
        Err(why) => {
            panic!("Couldn't read to string: {}", why)
        }
    };
    let parts: Vec<_> = data.trim().split("\n\n").collect();
    let mut bingo_boards: Vec<Board> = Vec::new();

    let nums: Vec<i32> = parts[0]
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    for part in &parts[1..] {
        bingo_boards.push(Board::from(*part));
    }

    (nums, bingo_boards)
}
