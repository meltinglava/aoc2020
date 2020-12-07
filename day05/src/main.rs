 #![warn(
     clippy::all,
     clippy::restriction,
     clippy::pedantic,
     clippy::nursery,
     clippy::cargo,
 )]

use aoc_tool::{aoc_tester, read_input, read_input_lines};

type TaskType = Vec<usize>;

fn parse(inp: Vec<String>) -> TaskType {
    inp.iter()
        .map(|seat| {
            seat.chars()
                .map(|c| match c {
                    'B' | 'R' => '1',
                    'F' | 'L' => '0',
                    s => unreachable!("Unknown value: {}", s),
                })
                .collect::<String>()
        })
        .map(|num| usize::from_str_radix(&num, 2).unwrap())
        .collect()
}

fn part1(inp: TaskType) -> usize {
    inp.into_iter().max().unwrap()
}

fn part2(inp: TaskType, part1: usize) -> usize {
    let mut v = vec![false; part1 + 1];
    inp.into_iter().for_each(|n| v[n] = true);
    v.windows(3)
        .enumerate()
        .find(|(_, a)| a == &[true, false, true])
        .map(|(v, _)| v + 1) // enumerate starts with 0, but we check for the middle value
        .unwrap()
}

fn main() {
    let input = parse(read_input_lines::<Vec<String>, String>());
    let p1 = part1(input.clone());
    println!("Part1: {}", p1);
    println!("Part2: {}", part2(input, p1));
}
