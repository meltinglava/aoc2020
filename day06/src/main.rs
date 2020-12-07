#![feature(iterator_fold_self)]

use std::collections::HashSet;

use aoc_tool::read_input;

fn part1(inp: &str) -> usize {
    inp.split("\n\n")
        .map(|info| {
            info.chars()
                .filter(char::is_ascii_lowercase)
                .collect::<HashSet<_>>()
        })
        .map(|set| set.len())
        .sum()
}

fn part2(inp: &str) -> usize {
    inp.split("\n\n")
        .map(|lines| {
            lines
                .lines()
                .map(|line| line.chars().collect::<HashSet<_>>())
                .fold_first(|a, b| a.intersection(&b).cloned().collect::<HashSet<char>>())
                .unwrap()
                .len()
        })
        .sum()
}

fn main() {
    let input = read_input();
    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}
