use std::ops::RangeInclusive;

use aoc_tool::{aoc_tester, read_input, read_input_lines};

type TaskType = Vec<PassPolicy>; //TODO: Change me to the type

#[derive(Debug, Clone)]
struct PassPolicy {
    start: usize,
    end: usize,
    letter: char,
    pass: String,
}

fn parse(inp: String) -> PassPolicy {
    // 1-3 c: pass
    let vals: Vec<_> = inp.split(' ').collect();
    let rng_nums: Vec<usize> = vals[0]
        .split('-')
        .map(str::parse)
        .map(|n| n.unwrap())
        .collect();
    let start = rng_nums[0];
    let end = rng_nums[1];
    let letter = vals[1].chars().next().unwrap();
    let pass = vals[2].to_string();
    PassPolicy { start, end, letter, pass }
}

fn part1(inp: TaskType) -> usize {
    inp.into_iter()
        .filter(|pp| (pp.start..=pp.end).contains(&pp.pass.matches(pp.letter).count()))
        .count()
}

fn part2(inp: TaskType) -> usize {
    inp.into_iter()
       .filter(|pp| {
           let start = pp.start - 1;
           let end = pp.end - 1;
           pp.pass.chars().enumerate().filter(|(n, _)| *n == start || *n == end)
               .filter(|(_, n)| *n == pp.letter).count() == 1
       })
        .count()
}

fn main() {
    let input: Vec<_> = read_input_lines::<Vec<String>, String>()
        .into_iter()
        .map(|n| parse(n))
        .collect();
    println!("Part1: {}", part1(input.clone()));
    println!("Part2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        aoc_tester(
            vec![(
                vec!["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"]
                    .into_iter()
                    .map(|n| parse(n.to_string()))
                    .collect::<TaskType>(),
                2,
            )],
            part1,
        );
    }

    #[test]
    fn test_part2() {
        aoc_tester(
            vec![(
                vec!["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"]
                    .into_iter()
                    .map(|n| parse(n.to_string()))
                    .collect::<TaskType>(),
                1,
            )],
            part2,
        );
    }
}
