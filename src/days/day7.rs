use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};

use super::common::Grid;

#[derive(Debug, Clone, Copy)]
pub enum TachyonManifold {
    EmptySpace,
    Splitter,
}

#[aoc_generator(day7)]
pub fn aoc_generator(input: &str) -> (usize, Grid<TachyonManifold>) {
    let start = input
        .lines()
        .next()
        .expect("no start line")
        .chars()
        .position(|x| x == 'S')
        .expect("no start position found");

    let grid = Grid::new(
        input
            .lines()
            .skip(1)
            .map(|line| {
                line.chars()
                    .map(|char| match char {
                        '.' => TachyonManifold::EmptySpace,
                        '^' => TachyonManifold::Splitter,
                        _ => panic!("invalid grid item"),
                    })
                    .collect()
            })
            .collect(),
    );
    (start, grid)
}

#[aoc(day7, part1)]
pub fn part1((start, grid): &(usize, Grid<TachyonManifold>)) -> i64 {
    let mut lightbeams = HashSet::new();
    let mut splits = 0;
    lightbeams.insert((0, *start));
    'outer: loop {
        let mut new_lightbeams = HashSet::new();
        for (row, column) in lightbeams {
            match grid.get(row, column) {
                None => break 'outer,
                Some(TachyonManifold::Splitter) => {
                    splits += 1;
                    new_lightbeams.insert((row + 1, column - 1));
                    new_lightbeams.insert((row + 1, column + 1));
                }
                Some(TachyonManifold::EmptySpace) => {
                    new_lightbeams.insert((row + 1, column));
                }
            };
        }
        lightbeams = new_lightbeams;
    }
    splits
}

#[aoc(day7, part2)]
pub fn part2((start, grid): &(usize, Grid<TachyonManifold>)) -> i64 {
    let mut lightbeams = HashMap::new();
    let mut result = 0;
    lightbeams.insert((0_usize, *start), 1_i64);
    'outer: loop {
        let mut new_lightbeams = HashMap::new();
        for ((row, column), amount) in lightbeams {
            match grid.get(row, column) {
                None => break 'outer,
                Some(TachyonManifold::Splitter) => {
                    new_lightbeams
                        .entry((row + 1, column - 1))
                        .and_modify(|existing| *existing += amount)
                        .or_insert(amount);
                    new_lightbeams
                        .entry((row + 1, column + 1))
                        .and_modify(|existing| *existing += amount)
                        .or_insert(amount);
                }
                Some(TachyonManifold::EmptySpace) => {
                    new_lightbeams
                        .entry((row + 1, column))
                        .and_modify(|existing| *existing += amount)
                        .or_insert(amount);
                }
            };
        }
        result = new_lightbeams.clone().into_values().sum();
        lightbeams = new_lightbeams;
    }
    result
}
