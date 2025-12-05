use aoc_runner_derive::{aoc, aoc_generator};

use super::common::Grid;

#[derive(Clone, Copy, Debug)]
pub enum StorageLocation {
    PaperRoll,
    Empty,
}

#[aoc_generator(day4)]
pub fn generator(input: &str) -> Grid<StorageLocation> {
    let cells = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| {
                    if c == '@' {
                        StorageLocation::PaperRoll
                    } else {
                        StorageLocation::Empty
                    }
                })
                .collect()
        })
        .collect();
    Grid::new(cells)
}

#[aoc(day4, part1)]
pub fn part1(input: &Grid<StorageLocation>) -> i32 {
    let mut sum = 0;
    for ((i, j), &value) in input.cells() {
        if matches!(value, StorageLocation::Empty) {
            continue;
        }
        let neighbor_paper_rolls = input.neighbors(i, j).iter().fold(0, |acc, s| {
            if matches!(s.2, StorageLocation::PaperRoll) {
                return acc + 1;
            }
            acc
        });
        if neighbor_paper_rolls < 4 {
            sum += 1;
        }
    }
    sum
}

#[aoc(day4, part2)]
pub fn part2(input: &Grid<StorageLocation>) -> i32 {
    let mut input = input.clone();
    let mut sum = 0;
    loop {
        let mut updates = Vec::new();
        for ((i, j), value) in input.cells() {
            if matches!(value, StorageLocation::Empty) {
                continue;
            }
            let neighbor_paper_rolls = input.neighbors(i, j).iter().fold(0, |acc, s| {
                acc + matches!(s.2, StorageLocation::PaperRoll) as i32
            });
            if neighbor_paper_rolls < 4 {
                updates.push((i, j));
            }
        }
        if updates.is_empty() {
            break;
        }
        sum += updates.len() as i32;
        for (i, j) in updates {
            input.replace(i, j, StorageLocation::Empty);
        }
    }
    sum
}
