use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
pub struct SafeInstruction {
    direction: Direction,
    amount: i32,
}

#[aoc_generator(day1)]
pub fn generator(input: &str) -> Vec<SafeInstruction> {
    input
        .lines()
        .map(|line| {
            let line = line.trim();
            if line.is_empty() {
                return SafeInstruction {
                    direction: Direction::Left,
                    amount: 0,
                };
            }
            let dir_char = line.chars().next().expect("Line to short");
            let direction = match dir_char {
                'L' => Direction::Left,
                'R' => Direction::Right,
                _ => panic!("Invalid direction: {}", dir_char),
            };
            let amount_str = &line[1..];
            let amount = amount_str.parse().expect("Invalid number");

            SafeInstruction { direction, amount }
        })
        .collect()
}

#[aoc(day1, part1)]
pub fn part1(instructions: &[SafeInstruction]) -> i32 {
    let mut sum = 50;
    let mut zero_count = 0;

    for instruction in instructions {
        match instruction.direction {
            Direction::Left => sum -= instruction.amount,
            Direction::Right => sum += instruction.amount,
        }
        sum = sum.rem_euclid(100);
        if sum == 0 {
            zero_count += 1;
        }
    }

    zero_count
}

#[aoc(day1, part2)]
pub fn part2(instructions: &[SafeInstruction]) -> i32 {
    let mut sum = 50;
    let mut zero_count = 0;

    for instruction in instructions {
        for _i in 0..instruction.amount {
            match instruction.direction {
                Direction::Left => sum -= 1i32,
                Direction::Right => sum += 1i32,
            }
            sum = sum.rem_euclid(100);
            if sum == 0 {
                zero_count += 1;
            }
        }
    }

    zero_count
}
