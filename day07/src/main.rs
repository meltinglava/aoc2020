#![feature(str_split_once)]
use std::collections::{HashMap, HashSet};

use aoc_tool::read_input_lines;

type TaskType = HashMap<String, HashMap<String, usize>>;

fn parse(inp: Vec<String>) -> TaskType {
    inp.into_iter()
        .map(|line| {
            let line = line.trim_end_matches(|c: char| c.is_ascii_punctuation()); //removes last '.'
            let (bag, contains) = line.split_once(" bags contain ").unwrap();
            (
                bag.to_string(),
                match contains {
                    "no other bags" => HashMap::new(),
                    b => b
                        .split(", ")
                        .map(|bag_info| {
                            bag_info
                                .trim_end_matches(" bags")
                                .trim_end_matches(" bag")
                                .split_once(' ')
                                .unwrap()
                        })
                        .map(|(n, bag)| (bag.to_string(), n.parse().unwrap()))
                        .collect(),
                },
            )
        })
        .collect()
}

fn bag_contains(bag: &str, bags: &TaskType, visited: &mut HashSet<String>) -> bool {
    for b in bags[bag].keys() {
        if b == "shiny gold" {
            return true;
        }
        if visited.get(b).is_none() {
            visited.insert(b.to_string());
            if bag_contains(b, bags, visited) {
                return true;
            }
        }
    }
    false
}

fn part1(inp: &TaskType) -> usize {
    inp.iter()
        .filter(|c| c.0 != "shiny gold")
        .filter(|(b, _)| {
            let mut visited = HashSet::new();
            bag_contains(b, inp, &mut visited)
        })
        .count()
}

fn bags_in_bag(bag: &str, bags: &TaskType) -> usize {
    bags[bag]
        .iter()
        .map(|(bag, n)| n * (bags_in_bag(bag, bags) + 1))
        .sum()
}

fn part2(inp: &TaskType) -> usize {
    bags_in_bag("shiny gold", &inp)
}

fn main() {
    let input = parse(read_input_lines::<Vec<String>, String>());
    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::{assert_equal, Itertools};
    use maplit::hashmap;

    const TEST_CASE_1: &'static str = "\
    light red bags contain 1 bright white bag, 2 muted yellow bags.\n\
    dark orange bags contain 3 bright white bags, 4 muted yellow bags.\n\
    bright white bags contain 1 shiny gold bag.\n\
    muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.\n\
    shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.\n\
    dark olive bags contain 3 faded blue bags, 4 dotted black bags.\n\
    vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.\n\
    faded blue bags contain no other bags.\n\
    dotted black bags contain no other bags.";

    fn sort_tester(
        v: HashMap<String, HashMap<String, usize>>,
    ) -> Vec<(String, Vec<(String, usize)>)> {
        v.into_iter()
            .sorted_by_key(|(s, _)| s.to_string())
            .map(|(s, v)| {
                (s, {
                    v.into_iter()
                        .sorted_by_key(|(s, _)| s.to_string())
                        .collect()
                })
            })
            .collect()
    }

    #[test]
    fn test_parse() {
        let expected = hashmap! {
            "light red".to_string() => hashmap!{"bright white".to_string() => 1, "muted yellow".to_string() => 2},
            "dark orange".to_string() => hashmap!{"bright white".to_string() => 3, "muted yellow".to_string() => 4},
            "bright white".to_string() => hashmap!{"shiny gold".to_string() => 1},
            "muted yellow".to_string() => hashmap!{"shiny gold".to_string() => 2, "faded blue".to_string() => 9},
            "shiny gold".to_string() => hashmap!{"dark olive".to_string() => 1, "vibrant plum".to_string() => 2},
            "dark olive".to_string() => hashmap!{"faded blue".to_string() => 3, "dotted black".to_string() => 4},
            "vibrant plum".to_string() => hashmap!{"faded blue".to_string() => 5, "dotted black".to_string() => 6},
            "faded blue".to_string() => hashmap!{},
            "dotted black".to_string() => hashmap!{},
        };
        assert_equal(
            sort_tester(expected.clone()),
            sort_tester(parse(TEST_CASE_1.lines().map(str::to_string).collect())),
        );
        assert_eq!(
            dbg!(expected),
            dbg!(parse(TEST_CASE_1.lines().map(str::to_string).collect()))
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(&parse(TEST_CASE_1.lines().map(str::to_string).collect())),
            4
        )
    }

    const TEST_CASE_2: &'static str = "\
    shiny gold bags contain 2 dark red bags.\n\
    dark red bags contain 2 dark orange bags.\n\
    dark orange bags contain 2 dark yellow bags.\n\
    dark yellow bags contain 2 dark green bags.\n\
    dark green bags contain 2 dark blue bags.\n\
    dark blue bags contain 2 dark violet bags.\n\
    dark violet bags contain no other bags.";

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(&parse(TEST_CASE_2.lines().map(str::to_string).collect())),
            126
        )
    }
}
