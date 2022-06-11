use std::fs;

mod board;
use board::Board;
use board::BoardCollection;

fn main() {
    let filepath = "day4.in";
    let mut part1 = 0_u32;
    let mut last_winning_num = 0_u32;
    let (nums_drawn, mut bingo_boards) = read_input(filepath);

    // part 1
    // 'numbers_draw: for num in &nums_drawn {
    //     bingo_boards.mark_boards(*num);
    //     for mut board in bingo_boards.boards() {
    //         if board.is_winner() {
    //             part1 = board.calculate_winning_score(*num);
    //             break 'numbers_draw;
    //         }
    //     }
    // }

    // println!("{}", part1);

    // part 2
    for (_, num) in nums_drawn.iter().enumerate() {
        bingo_boards.mark_boards(*num);

        if bingo_boards.all_completed() {
            last_winning_num = *num;
            break;
        }

        for mut board in bingo_boards.boards() {
            if board.is_winner() {
                bingo_boards.pop_winning_board(board)
            }
        }
    }

    let part2 = bingo_boards.last_completed_board();
    // .calculate_winning_score(last_winning_num);
    println!("{}", part2);
    println!("{}", last_winning_num);
    println!("{:?}", nums_drawn);
}

fn read_input(filepath: &str) -> (Vec<u32>, BoardCollection) {
    let data = match fs::read_to_string(filepath) {
        Ok(data) => data,
        Err(why) => {
            panic!("Couldn't read to string: {}", why)
        }
    };
    let parts: Vec<_> = data.trim().split("\n\n").collect();
    let mut bingo_boards: Vec<Board> = Vec::new();

    let nums: Vec<u32> = parts[0]
        .split(',')
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    for part in &parts[1..] {
        bingo_boards.push(Board::from(*part));
    }

    (nums, BoardCollection::from(bingo_boards))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn board_size() {
        let filename = "day4.in";
        let (_, bingo_boards) = read_input(filename);

        for board in bingo_boards.boards() {
            assert_eq!(board.board().len(), 25);
        }
    }

    #[test]
    fn marked_boards() {
        let filename = "day4_sample.in";
        let (_, mut bingo_boards) = read_input(filename);
        let num = vec![7, 4];

        bingo_boards.mark_boards(num[0]);
        bingo_boards.mark_boards(num[1]);
        let mut vec1 = vec![false; 25];
        vec1[14] = true;
        vec1[8] = true;
        let mut vec2 = vec![false; 25];
        vec2[12] = true;
        vec2[19] = true;
        let mut vec3 = vec![false; 25];
        vec3[24] = true;
        vec3[4] = true;
        assert_eq!(bingo_boards.boards()[0].marked(), vec1);
        assert_eq!(bingo_boards.boards()[1].marked(), vec2);
        assert_eq!(bingo_boards.boards()[2].marked(), vec3);
    }

    #[test]
    fn part1_sample() {
        let filename = "day4_sample.in";
        let mut answer: u32 = 0;
        let (nums, mut bingo_boards) = read_input(filename);

        'draw_nums: for num in nums {
            bingo_boards.mark_boards(num);
            for mut board in bingo_boards.boards() {
                if board.is_winner() {
                    answer = board.calculate_winning_score(num);
                    break 'draw_nums;
                }
            }
        }
        assert_eq!(answer, 4512)
    }

    #[test]
    fn part1() {
        let filename = "day4.in";
        let mut answer: u32 = 0;
        let (nums, mut bingo_boards) = read_input(filename);

        'draw_nums: for num in nums {
            bingo_boards.mark_boards(num);
            for mut board in bingo_boards.boards() {
                if board.is_winner() {
                    answer = board.calculate_winning_score(num);
                    break 'draw_nums;
                }
            }
        }
        assert_eq!(answer, 39984)
    }

    fn part2() {
        let filename = "day4_sample.in";
        let mut answer: u32 = 0;
        let mut winning_num: u32 = 0;
        let (nums, mut bingo_boards) = read_input(filename);

        'outer: for num in nums {
            bingo_boards.mark_boards(num);
            if bingo_boards.all_completed() {
                println!("{}", num);
                break 'outer;
            }
        }
    }
}
