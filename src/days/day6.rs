use std::{cmp::max, vec::IntoIter};

use aoc_runner_derive::aoc;

#[derive(Debug, Clone, Copy)]
pub enum Operation {
    Multiply,
    Sum,
}

#[aoc(day6, part1)]
pub fn part1(input: &str) -> i64 {
    let lines = input.lines().collect::<Vec<&str>>();
    let operations: Vec<Operation> = lines
        .clone()
        .last()
        .expect("no operation line")
        .split_whitespace()
        .map(|f| match f {
            "*" => Operation::Multiply,
            "+" => Operation::Sum,
            _ => panic!("invalid operation: \"{:}\"", f),
        })
        .collect();

    let numbers: Vec<Vec<i64>> = lines
        .iter()
        .rev()
        .skip(1)
        .rev()
        .map(|line| {
            line.split_whitespace()
                .map(|number| number.parse().expect("invalid number"))
                .collect()
        })
        .collect();

    let mut sum = 0;
    let mut iters: Vec<std::vec::IntoIter<i64>> =
        numbers.iter().cloned().map(|n| n.into_iter()).collect();
    for operation in operations.iter() {
        let mut inner_sum = 0;
        for iter in iters.iter_mut() {
            match operation {
                Operation::Multiply => {
                    inner_sum = max(1, inner_sum) * iter.next().expect("not enough length")
                }
                Operation::Sum => inner_sum += iter.next().expect("not enough length"),
            }
        }
        println!("{:}", inner_sum);
        sum += inner_sum;
    }
    sum
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> i64 {
    let mut sum = 0;
    let matrix: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    if matrix.len() < 2 {
        panic!("No correct input");
    }
    let mut numbers: Vec<IntoIter<char>> = matrix[..matrix.len() - 1]
        .iter()
        .cloned()
        .map(Vec::into_iter)
        .collect();
    let operations: Vec<Operation> = matrix
        .last()
        .expect("no last row")
        .iter()
        .collect::<String>()
        .split_whitespace()
        .map(|t| match t {
            "*" => Operation::Multiply,
            "+" => Operation::Sum,
            _ => panic!("invalid operation"),
        })
        .collect();

    for op in operations {
        let mut inner_sum = 0;
        loop {
            let mut number = 0;
            for row in numbers.iter_mut() {
                if let Some(value) = row.next() {
                    if let Ok(int_value) = value.to_string().parse::<i64>() {
                        number = number * 10 + int_value
                    }
                }
            }
            if number == 0 {
                break;
            }
            match op {
                Operation::Multiply => inner_sum = max(1, inner_sum) * number,
                Operation::Sum => inner_sum += number,
            }
        }
        sum += inner_sum;
    }
    sum
}
