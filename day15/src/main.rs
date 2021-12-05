use std::collections::HashMap;

fn game(input: &[usize], target: usize) -> usize {
    let mut spoken = HashMap::new();
    input
        .iter()
        .copied()
        .take(input.len() - 1)
        .enumerate()
        .for_each(|(n, v)| {spoken.insert(v, n);});
    let mut last = *input.last().unwrap();
    for n in input.len()..target {
        let next = match spoken.get(&last) {
            Some(l) => n - 1 - l,
            None => 0,
        };
        spoken.insert(last, n - 1);
        last = next;
    }
    last
}

fn main() {
    println!("Part 1: {}", game(&[16,1,0,18,12,14,19], 2020));
    println!("Part 2: {}", game(&[16,1,0,18,12,14,19], 30000000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(game(&[0,3,6], 2020), 436);
        assert_eq!(game(&[1,3,2], 2020), 1);
        assert_eq!(game(&[2,1,3], 2020), 10);
        assert_eq!(game(&[1,2,3], 2020), 27);
        assert_eq!(game(&[2,3,1], 2020), 78);
        assert_eq!(game(&[3,2,1], 2020), 438);
        assert_eq!(game(&[3,1,2], 2020), 1836);
    }

    #[test]
    fn test_part2() {
        assert_eq!(game(&[0,3,6], 30000000), 175594);
        assert_eq!(game(&[1,3,2], 30000000), 2578);
        assert_eq!(game(&[2,1,3], 30000000), 3544142);
        assert_eq!(game(&[1,2,3], 30000000), 261214);
        assert_eq!(game(&[2,3,1], 30000000), 6895259);
        assert_eq!(game(&[3,2,1], 30000000), 18);
        assert_eq!(game(&[3,1,2], 30000000), 362);
    }
}
