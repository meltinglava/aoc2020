use std::{str::FromStr, collections::HashMap, convert::TryFrom, convert::TryInto, num::ParseIntError, ops::Deref, ops::RangeInclusive, string::ParseError};

use aoc_tool::{aoc_tester, read_input, read_input_lines};

use thiserror::Error;

type TaskType = Vec<Result<PassPort, PassPortError>>; //TODO: Change me to the type

#[derive(Debug, Clone)]
struct PassPort {
    byr: usize,         // (Birth Year)
    iyr: usize,         // (Issue Year)
    eyr: usize,         // (Expiration Year)
    hgt: Height,         // (Height)
    hcl: HairColor,         // (Hair Color)
    ecl: EyeColor,         // (Eye Color)
    pid: Pid,         // (Passport ID)
    cid: Option<String>, // (Country ID)
}

#[derive(Debug, Clone)]
struct Pid {
    id: String
}

impl FromStr for Pid {
    type Err = PassPortError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 9 {
            return Err(PassPortError::FieldMissing("Wrong lenth pid".to_string()))
        }
        let _a: usize = s.parse()?;
        Ok(Self{id: s.to_string()})
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum EyeColor {
    Amb,
    Blu,
    Brn,
    Gry,
    Grn,
    Hzl,
    Oth,
}

impl FromStr for EyeColor {
    type Err = PassPortError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "amb" => Ok(Self::Amb),
            "blu" => Ok(Self::Blu),
            "brn" => Ok(Self::Brn),
            "gry" => Ok(Self::Gry),
            "grn" => Ok(Self::Grn),
            "hzl" => Ok(Self::Hzl),
            "oth" => Ok(Self::Oth),
            n => Err(PassPortError::FieldMissing(format!("{}: is mot a color", n)))
        }
    }
}

#[derive(Debug, Clone)]
struct HairColor {
    color: usize,
}

impl FromStr for HairColor {
    type Err = PassPortError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vals = s.split_at(1);
        if vals.0 != "#" {
            return Err(PassPortError::FieldMissing("Missing starting # in color".to_string()));
        }
        if vals.1.len() != 6 {
            return Err(PassPortError::FieldMissing("Wrong number of digits in color".to_string()));
        }
        Ok(Self{color: usize::from_str_radix(vals.1, 16)?})
    }
}

#[derive(Debug, Clone)]
enum Height {
    Cm(usize),
    In(usize),
}

impl FromStr for Height {
    type Err = PassPortError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let len = s.len();
        let fields = s.split_at(len - 2);
        let number: usize = fields.0.parse()?;
        let unit = fields.1;
        match unit {
            "cm" => Ok(Height::Cm(PassPortError::in_range(150..=193, number)?)),
            "in" => Ok(Height::In(PassPortError::in_range(59..=76, number)?)),
            n => Err(PassPortError::FieldMissing("Malformed field".to_string()))
        }
    }
}

#[derive(Error, Debug, Clone)]
enum PassPortError {
    #[error("Do not have any data for: `{0}`")]
    FieldMissing(String),
    #[error("Value: `{1}`, is outisde of range: {0:?}")]
    NotInRangeError(RangeInclusive<usize>, usize),
    #[error("Data cant be an int")]
    NumberError(#[from] ParseIntError)
}

impl PassPortError {
    fn in_range(rng: RangeInclusive<usize>, num: usize) -> Result<usize, PassPortError> {
        if rng.contains(&num) {
            Ok(num)
        } else {
            Err(PassPortError::NotInRangeError(rng, num))
        }
    }
}


impl TryFrom<HashMap<String, String>> for PassPort {
    type Error = PassPortError;

    fn try_from(map: HashMap<String, String>) -> Result<Self, Self::Error> {
        Ok(PassPort {
            byr: PassPortError::in_range(1920..=2002, map
                .get("byr")
                .ok_or_else(|| PassPortError::FieldMissing("byr".to_string()))?
                .parse()?)?,
            iyr: PassPortError::in_range(2010..=2020, map
                .get("iyr")
                .ok_or_else(|| PassPortError::FieldMissing("iyr".to_string()))?
                .parse()?)?,
            eyr: PassPortError::in_range(2020..=2030, map
                .get("eyr")
                .ok_or_else(|| PassPortError::FieldMissing("eyr".to_string()))?
                .parse()?)?,
            hgt: map
                .get("hgt")
                .ok_or_else(|| PassPortError::FieldMissing("hgt".to_string()))?
                .parse()?,
            hcl: map
                .get("hcl")
                .ok_or_else(|| PassPortError::FieldMissing("hcl".to_string()))?
                .parse()?,
            ecl: map
                .get("ecl")
                .ok_or_else(|| PassPortError::FieldMissing("ecl".to_string()))?
                .parse()?,
            pid: map
                .get("pid")
                .ok_or_else(|| PassPortError::FieldMissing("pid".to_string()))?
                .parse()?,
            cid: map.get("cid").map(|n| n.to_string()),
        })
    }
}

fn parse(inp: String) -> TaskType {
    let mut map = HashMap::new();
    inp.lines()
        .filter_map(|line| match line {
            l if l.is_empty() => {
                let data = std::mem::replace(&mut map, HashMap::new());
                Some(data.try_into())
            }
            _ => {
                line.split_whitespace()
                    .map(|d| d.split(':').map(|s| s.to_string()).collect::<Vec<_>>())
                    .for_each(|v| {
                        map.insert(v[0].clone(), v[1].clone());
                    });
                None
            }
        })
        .collect()
}

fn part1(inp: TaskType) -> usize {
    inp.iter().filter(|&v| v.is_ok()).count()
}

fn part2(inp: TaskType) -> usize {
    inp.iter().filter(|&v| v.is_ok()).count()
}

fn main() {
    let mut input = read_input();
    input.push('\n');
    let input = parse(input);
    //println!("Part1: {}", part1(input.clone()));
    println!("Part2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hegith() {
        assert!("60in".parse::<Height>().is_ok());
        assert!("190cm".parse::<Height>().is_ok());
        assert!("190in".parse::<Height>().is_err());
        assert!("60".parse::<Height>().is_err());
    }
}
