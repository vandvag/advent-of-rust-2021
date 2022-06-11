mod line;
mod point;

use line::Line;

use std::fs;

const BOARD_SIZE: usize = 1000;

fn main() {
    // Read input
    let filename = "day5.in";
    let data: Vec<String> = match fs::read_to_string(filename) {
        Ok(data) => data.split('\n').map(|x| x.to_string()).collect(),
        Err(why) => panic!("Couldn't read file {}", why),
    };
    let mut lines: Vec<Line> = Vec::new();
    for d in data.into_iter() {
        lines.push(Line::from(d));
    }
    let mut max_dim = 0;

    for line in &lines {
        max_dim = [max_dim, line.start().x(), line.start().y(), line.end().x(), line.end().y()].into_iter().max().unwrap();
    }

    let mut board = [[0_u32; BOARD_SIZE]; BOARD_SIZE];
    // let mut map: HashMap<(i32, i32), usize> = HashMap::new();

    for line in lines {
        // horizontal -> start.y == end.y
        if line.is_horizontal() {
            let (startpoint, endpoint) = minmax(line.start().x(), line.end().x());
            for i in startpoint..=endpoint {
                board[i as usize][line.start().y() as usize] += 1;
            }
        }

        // vertical -> start.x == end.x
        if line.is_vertical() {
            let (startpoint, endpoint) = minmax(line.start().y(), line.end().y());
            for i in startpoint..=endpoint {
                board[line.start().x() as usize][i as usize] += 1;
            }
        }

        if line.is_diagonal() {
            let line_size = (line.start().x() - line.end().x()).abs();
            let x_direction = match line.start().x() - line.end().x() < 0 {
                true => 1,
                false => -1,
            };
            let y_direction = match line.start().y() - line.end().y() < 0 {
                true => 1,
                false => -1,
            };
            for i in 0..=line_size {
                board[(line.start().x() + i * x_direction) as usize][(line.start().y() + i * y_direction) as usize] += 1;
            }
        }
    }

    let mut sum = 0;
    for (i, b) in board.iter().enumerate() {
        for (j, _) in b.iter().enumerate() {
            if board[i][j] >= 2 {
                sum += 1;
            }
        }
    }
    println!("{}", sum);
}


fn minmax(a: i32, b: i32) -> (i32, i32) {
    if a > b { (b, a) }
    else { (a, b) }
}