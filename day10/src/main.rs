use itertools::Itertools;

use aoc_tool::read_input_lines;

type TaskType = Vec<usize>;

fn part1(v: TaskType) -> usize {
    let mut last = 0;
    let mut one = 0;
    let mut three = 0;
    for i in v {
        match i - last {
            1 => one += 1,
            3 => three += 1,
            n => println!("{}", n),
        }
        last = i;
    }
    one * three + 1
}

fn part2(v: TaskType) -> usize {
    let skips: Vec<_> = std::iter::once(0)
        .chain(v.to_vec())
        .chain(std::iter::once(v.last().unwrap() + 3))
        .tuple_windows::<(_, _)>()
        .map(|(f, n)| n - f)
        .collect();
    skips
        .split(|&n| n == 3)
        .filter(|n| !n.is_empty())
        .map(|n| n.len())
        .map(|n| match n {
            5 => 30,
            4 => 7,
            1..=3 => 2usize.pow((n - 1) as u32),
            0 => unreachable!("We filter empty"),
            n => unimplemented!("does not support higher than 5, given: {}", n),
        })
        .product()
}

fn main() {
    let mut input = read_input_lines::<TaskType, usize>();
    input.sort_unstable();
    let p1 = part1(input.clone());
    println!("Part1: {}", p1);
    println!("Part2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_large() {
        let mut test_val = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];
        test_val.sort_unstable();
        assert_eq!(part2(dbg!(test_val)), 19208)
    }

    #[test]
    fn test_small() {
        let mut test_val = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        test_val.sort_unstable();
        assert_eq!(part2(dbg!(test_val)), 8)
    }
}
