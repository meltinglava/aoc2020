use aoc_tool::{aoc_tester, read_input, read_input_lines};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum AreaType {
    Tree,
    Open,
}

type TaskType = Vec<Vec<AreaType>>; //TODO: Change me to the type

fn parse(inp: String) -> Vec<AreaType> {
    inp.chars()
        .map(|a| match a {
            '#' => AreaType::Tree,
            '.' => AreaType::Open,
            _ => unreachable!("AreaType: {}, is unknown", a),
        })
        .collect()
}

fn part1(inp: TaskType) -> usize {
    inp.into_iter()
        .enumerate()
        .map(|(l, r)| {
            let len = r.len();
            (r, (l * 3) % len)
        })
        .map(|(r, c)| r[c])
        .filter(|a| *a == AreaType::Tree)
        .count()
}

fn part2(inp: TaskType, to_check: &[(usize, usize)]) -> usize {
    to_check
        .iter()
        .cloned()
        .map(|(right, down)| {
            inp.iter()
                .cloned()
                .step_by(down)
                .enumerate()
                .map(|(l, row)| {
                    let len = row.len();
                    (row, (l * right) % len)
                })
                .map(|(r, c)| r[c])
                .filter(|a| *a == AreaType::Tree)
                .count()
        })
        .product()
}

fn main() {
    let input: Vec<_> = read_input_lines::<Vec<String>, String>()
        .into_iter()
        .map(parse)
        .collect();
    println!("Part1: {}", part1(input.clone()));
    println!(
        "Part2: {}",
        part2(input, &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)])
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        aoc_tester(
            vec![(
                vec![1721usize, 979, 366, 299, 675, 1456]
                    .into_iter()
                    .collect(),
                514579,
            )],
            part1,
        );
    }

    #[test]
    fn test_part2() {
        aoc_tester(
            vec![(
                vec![1721usize, 979, 366, 299, 675, 1456]
                    .into_iter()
                    .collect(),
                241861950,
            )],
            part2,
        );
    }
}
