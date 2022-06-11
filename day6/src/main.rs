use std::fs;

fn main() {
    let filename = "day6.in";
    let fish: Vec<u128> = fs::read_to_string(filename)
        .unwrap()
        .split(',')
        .map(|x| x.parse::<u128>().unwrap())
        .collect();

    println!("{}", part1(&fish, 80));
    println!("{}", part1(&fish, 256));
}

fn part1(fish: &Vec<u128>, days: usize) -> u128 {
    let mut states = [0_u128; 9];
    for f in fish {
        states[*f as usize] += 1;
    }

    for _ in 0..days {
        // move everything one position to the left
        // aka the fish day is reduced by 1
        states.rotate_left(1);

        // after the shift the fish at states[8] have age = 0
        // so they need to be reset
        states[6] += states[8];
    }
    println!("{:?}", states);
    states.iter().sum::<u128>()
}
