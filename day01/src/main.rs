use std::collections::HashSet;

use itertools::*;

use aoc_tool::{aoc_tester, read_input, read_input_lines};

type TaskType = Vec<usize>; //TODO: Change me to the type

fn parse(inp: String) -> TaskType {
    todo!();
}

fn part1(inp: TaskType) -> usize {
    let set = inp.iter().collect::<HashSet<_>>();
    set.iter()
        .filter_map(|&i| Some((i, 2020usize.checked_sub(*i)?)))
        .find(|(_, v)| set.contains(v))
        .map(|(&i, v)| i * v)
        .unwrap()
}

fn part2(inp: TaskType) -> usize {
    let set = inp.iter().collect::<HashSet<_>>();
    set.iter()
        .cloned()
        .combinations(2)
        .map(|vals| (vals[0], vals[1]))
        .filter(|&(a, b)| a + b <= 2020)
        .filter_map(|(a, b)| Some((a, b, 2020usize.checked_sub(a + b)?)))
        .find(|(_, _, v)| set.contains(v))
        .map(|(a, b, c)| a * b * c)
        .unwrap()
}

fn main() {
    let input = read_input_lines::<usize>();
    println!("Part1: {}", part1(input.clone()));
    println!("Part2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        aoc_tester(vec![(vec![1721, 979, 366, 299, 675, 1456], 514579)], part1);
    }

    #[test]
    fn test_part2() {
        aoc_tester(
            vec![(vec![1721, 979, 366, 299, 675, 1456], 241861950)],
            part2,
        );
    }
}
