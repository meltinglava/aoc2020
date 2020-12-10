use combinations::Combinations;

use aoc_tool::read_input_lines;

type TaskType = Vec<usize>;


fn part1(v: TaskType) -> usize {
    v.windows(26).find_map(|window| -> Option<usize> {
        let contains_sum = Combinations::new(window[..25].to_vec(), 2)
            .map(|v| v.into_iter().sum())
            .any(|ref s: usize| *s == window[25]);
        if contains_sum {
            None
        } else {
            Some(window[25])
        }
    })
        .unwrap()
}

fn part2(v: TaskType, weak_number: usize) -> usize {
    for i in 0..v.len() {
        let mut a = 0;
        let mut large = 0;
        let mut small = usize::MAX;
        for n in v[i..].iter() {
            let n = *n;
            if n == weak_number {
                panic!("continious sum cant be it self")
            }
            if n < small {
                small = n
            }
            if n > large {
                large = n
            }
            a += n;
            if a == weak_number {
                return small + large;
            } else if a > weak_number{
                break;
            }
        }
    }
    unreachable!();
}

fn main() {
    let input = read_input_lines::<TaskType, usize>();
    let p1 = part1(input.clone());
    println!("Part1: {}", p1);
    println!("Part2: {}", part2(input, p1));
}
