use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
pub fn generator(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse().expect("number character"))
                .collect()
        })
        .collect()
}

fn find_value(bank: &[i64]) -> (i64, usize) {
    let mut max_value = 0;
    let mut max_index = 0;

    bank.iter().enumerate().for_each(|(i, &value)| {
        if value > max_value {
            max_value = value;
            max_index = i;
        }
    });
    (max_value, max_index)
}

fn runner(input: &[Vec<i64>], digits: usize) -> i64 {
    let mut sum = 0;
    for bank in input {
        if bank.is_empty() {
            continue;
        }
        let mut start = 0;
        let mut value = 0;
        for i in (0..digits).rev() {
            let (new_value, index) = find_value(&bank[start..bank.len() - i]);
            start = start + index + 1;
            value = value * 10 + new_value;
        }
        sum += value;
    }
    sum
}

#[aoc(day3, part1)]
pub fn part1(input: &[Vec<i64>]) -> i64 {
    runner(input, 2)
    // let mut sum = 0;
    // for bank in input {
    //     if bank.is_empty() {
    //         continue;
    //     }
    //     let (max_value_left, max_index_left) = find_value(&bank[..bank.len() - 1]);
    //
    //     let (max_value_right, _) = find_value(&bank[max_index_left + 1..]);
    //
    //     sum += max_value_left * 10 + max_value_right
    // }
    // sum
}

#[aoc(day3, part2)]
pub fn part2(input: &[Vec<i64>]) -> i64 {
    runner(input, 12)
}
