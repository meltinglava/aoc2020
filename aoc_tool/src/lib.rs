use std::fmt::Debug;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::iter::FromIterator;
use std::str::FromStr;

pub fn read_input() -> String {
    std::fs::read_to_string("input.txt").unwrap()
}

pub fn read_input_lines<C, T>() -> C
where
    C: FromIterator<T>,
    T: FromStr + Debug, <T as FromStr>::Err: Debug,
{
    let f = File::open("input.txt").unwrap();
    BufReader::new(f).lines().map(|l| l.unwrap()).map(|l| l.parse().unwrap()).collect::<C>()
}


pub fn aoc_tester<P, I, V, F>(paris: P, func: F)
where
    P: IntoIterator<Item = (I, V)>,
    I: Clone + Debug,
    V: PartialEq + Debug,
    F: FnOnce(I) -> V + Clone,
{
    paris
        .into_iter()
        .map(|(i, v)| (func.clone()(i.clone()), i, v))
        .for_each(|(actual, input_value, expected)| {
            assert!(
                actual == expected,
                "\nInput: {:?}\nCalculated value: {:?}\nExpected_value: {:?}\n",
                input_value,
                actual,
                expected
            )
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn double(n: usize) -> usize {
        n * 2
    }

    #[test]
    fn it_passes() {
        aoc_tester(vec![
            (2usize, 4usize),
            (3, 6),
        ], double)
    }

    #[test]
    #[should_panic]
    fn it_fails() {
        aoc_tester(vec![
            (2usize, 4usize),
            (3, 3),
        ], double)
    }
}
