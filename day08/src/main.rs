#![feature(str_split_once)]

use std::{collections::HashSet, num::ParseIntError, str::FromStr};

use aoc_tool::read_input;

#[derive(Debug, Clone)]
struct Computer {
    acc: isize,
    ip: usize,
    mem: Vec<Instruction>,
    running: bool,
}

impl Computer {
    fn run(&mut self) {
        if self.running {
            match self.mem.get(self.ip) {
                Some(i) => match i {
                    Instruction::Nop(_) => (),
                    Instruction::Acc(v) => self.acc += v,
                    Instruction::Jmp(v) => {
                        self.ip = ((self.ip as isize) + v) as usize;
                        return;
                    }
                },
                None => {
                    self.running = false;
                    return;
                }
            }
            self.ip += 1;
        }
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
enum Instruction {
    Jmp(isize),
    Acc(isize),
    Nop(isize),
}

impl FromStr for Instruction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (inst, val) = s.split_once(' ').unwrap();
        Ok(match inst {
            "jmp" => Self::Jmp(val.parse()?),
            "acc" => Self::Acc(val.parse()?),
            "nop" => Self::Nop(val.parse()?),
            e => unimplemented!("Unkown instruction: {}", e),
        })
    }
}

fn parse(s: &str) -> Computer {
    Computer {
        acc: 0,
        ip: 0,
        mem: s
            .lines()
            .map(Instruction::from_str)
            .map(|i| i.unwrap())
            .collect(),
        running: true,
    }
}

fn part1(mut comp: Computer) -> isize {
    let mut loop_detector = HashSet::new();
    while !loop_detector.contains(&comp.ip) {
        loop_detector.insert(comp.ip);
        comp.run();
    }
    comp.acc
}

fn part2(computer: Computer) -> isize {
    let mut skips = 0;
    while skips != computer.mem.len() {
        let mut comp = computer.clone();
        for inst in comp.mem.iter_mut().skip(skips) {
            skips += 1;
            let i = match inst {
                Instruction::Acc(_) => {
                    continue;
                }
                Instruction::Jmp(v) => Instruction::Nop(*v),
                Instruction::Nop(v) => Instruction::Jmp(*v),
            };
            let _ = std::mem::replace(inst, i);
            break;
        }
        let mut loop_detector = HashSet::new();
        while !loop_detector.contains(&comp.ip) && comp.running {
            loop_detector.insert(comp.ip);
            comp.run();
        }
        comp.run();
        if !comp.running {
            return comp.acc;
        }
    }
    unreachable!("No run found")
}

fn main() {
    let input = parse(&read_input());
    println!("Part1: {}", part1(input.clone()));
    println!("Part2: {}", part2(input));
}
