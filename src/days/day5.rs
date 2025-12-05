use std::ops::RangeInclusive;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day5)]
pub fn generator(input: &str) -> (Vec<RangeInclusive<i64>>, Vec<i64>) {
    let mut ranges = Vec::new();
    let mut ids = Vec::new();
    input.lines().for_each(|line| {
        if line.is_empty() {
            return;
        };
        let splits = line.split('-').collect::<Vec<&str>>();
        if splits.len() == 2 {
            let start = splits[0].parse().expect("Invalid start number");
            let end = splits[1].parse().expect("Invalid end number");
            ranges.push(start..=end)
        }
        if splits.len() == 1 {
            let number = splits[0].parse().expect("Invalid id");
            ids.push(number);
        }
    });
    (ranges, ids)
}

#[aoc(day5, part1)]
pub fn part1((ranges, ids): &(Vec<RangeInclusive<i64>>, Vec<i64>)) -> i64 {
    let mut sum = 0;

    for i in ids {
        for range in ranges {
            if range.contains(i) {
                sum += 1;
                break;
            }
        }
    }

    sum
}

#[aoc(day5, part2)]
pub fn part2((ranges, _ids): &(Vec<RangeInclusive<i64>>, Vec<i64>)) -> i64 {
    let mut mut_ranges = ranges.clone();
    mut_ranges.sort_by_key(|r| *r.start());

    let mut merged = Vec::new();
    let mut current = mut_ranges[0].clone();

    for r in mut_ranges.iter().skip(1) {
        if *r.start() <= *current.end() {
            if *r.end() > *current.end() {
                current = *current.start()..=*r.end();
            }
        } else {
            merged.push(current);
            current = r.clone();
        }
    }
    merged.push(current);

    let mut sum = 0;
    for r in merged {
        sum += r.count() as i64;
    }
    sum
}
