use std::ops::RangeInclusive;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2)]
pub fn generator(input: &str) -> Vec<RangeInclusive<i64>> {
    input
        .split(',')
        .map(|raw_range| {
            let numbers = raw_range.split('-').collect::<Vec<&str>>();
            if numbers.len() != 2 {
                panic!("length of numbers in range is not two")
            }
            let start = numbers[0].parse().expect("Invalid start number");
            let end = numbers[1].parse().expect("Invalid end number");
            start..=end
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn part1(ranges: &[RangeInclusive<i64>]) -> i64 {
    let mut sum = 0;
    for range in ranges {
        'number: for i in range.clone() {
            let string_numer = i.to_string();
            if string_numer.len() % 2 != 0 {
                continue 'number;
            }
            let half_lengt = string_numer.len() / 2;
            for j in 0..half_lengt {
                if string_numer.chars().nth(j) != string_numer.chars().nth(j + half_lengt) {
                    continue 'number;
                }
            }
            sum += i;
        }
    }
    sum
}

#[aoc(day2, part2)]
pub fn part2(ranges: &[RangeInclusive<i64>]) -> i64 {
    let mut sum = 0;
    for range in ranges {
        'number: for i in range.clone() {
            let string_numer = i.to_string();
            let lenght = string_numer.len();
            let mut j = 1;
            while j <= lenght / 2 {
                let pattern: String = string_numer.chars().take(j).collect();
                if string_numer.matches(pattern.as_str()).count() * j == lenght {
                    sum += i;
                    continue 'number;
                }
                j += 1;
            }
        }
    }
    sum
}
