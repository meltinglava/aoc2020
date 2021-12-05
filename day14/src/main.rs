use std::{collections::HashMap, fmt::Debug, fs::File, io::{self, BufRead, BufReader}, path::Path, str::FromStr};


#[derive(Clone)]
struct Mask {
    zero: usize,
    one: usize,
    x: Vec<usize>,
}

impl Debug for Mask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Mask").field("zero", &format!("{:#b}", self.zero)).field("one", &format!("{:#b}", self.one)).finish()
    }
}

impl Mask {
    fn new(mask: &str) -> Self {
        let mut zeroes = Vec::new();
        let mut ones = Vec::new();
        let mut x = Vec::new();
        mask.chars().rev().enumerate().for_each(|(n, f)| match f {
            '0' => zeroes.push(n),
            '1' => ones.push(n),
            'X' => x.push(n),
            u => unreachable!("Unknown mask: {}", u),
        });
        let mut zero = usize::MAX;
        let mut one = 0;

        for z in zeroes {
            zero ^= 1 << z;
        }

        for o in ones {
            one ^= 1 << o;
        }

        Self {
            zero,
            one,
            x,
        }
    }

    fn apply(&self, value: usize) -> usize {
        value & self.zero | self.one
    }

    fn mem_space(&self, mem: usize) -> Vec<usize> {
        let l = self.x.len();
        let mut addrs = Vec::new();
        for n in 0..(2usize.pow(l as u32)) {
            let mut z = usize::MAX;
            let mut o = 0;
            for (i, pos) in self.x.iter().enumerate() {
                match (n >> i) % 2 {
                    0 => z ^= 1 << pos,
                    1 => o ^= 1 << pos,
                    _ => unreachable!(),
                }
            }
            addrs.push(mem & z | o | self.one)
        }
        addrs
    }
}

#[derive(Debug, Clone)]
enum Task {
    Msk(Mask),
    Mem((usize, usize)),
}

impl FromStr for Task {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let l: Vec<_> = s.split(|c: char| !c.is_ascii_alphanumeric()).filter(|p| !p.is_empty()).collect();
        match l[0] {
            "mem" => Ok(Self::Mem((l[1].parse().unwrap(), l[2].parse().unwrap()))),
            "mask" => Ok(Self::Msk(Mask::new(l[1]))),
            n => Err(format!("Unknown operation: {}", n))
        }
    }
}

#[derive(Debug)]
struct Computer {
    mem: HashMap<usize, usize>,
}

impl Computer {
    fn new() -> Self {
        Self {
            mem: HashMap::new(),
        }
    }

    fn run(&mut self, addr: usize, value: usize) {
        *self.mem.entry(addr).or_default() = value;
    }

    fn sum(&self) -> usize {
        self.mem.iter().map(|(_, v)| v).copied().sum()
    }
}

fn parse_file<P: AsRef<Path>>(path: P) -> io::Result<Vec<Task>> {
    BufReader::new(File::open(path)?)
        .lines()
        .map(|l| Ok(l?.parse().unwrap()))
        .collect()
}

fn part1(input: &[Task]) -> usize {
    let mut instructions = input.iter();
    let mut mask = match instructions.next().unwrap() {
        Task::Msk(m) => m,
        Task::Mem(_) => unreachable!(),
    };
    let mut computer = Computer::new();
    for i in instructions {
        match i {
            Task::Msk(m) => mask = m,
            Task::Mem((addr, value)) => computer.run(*addr, mask.apply(*value)),
        }
    }
    computer.sum()
}

fn part2(input: &[Task]) -> usize {
    let mut instructions = input.iter();
    let mut mask = match instructions.next().unwrap() {
        Task::Msk(m) => m,
        Task::Mem(_) => unreachable!(),
    };
    let mut computer = Computer::new();
    for i in instructions {
        match i {
            Task::Msk(m) => mask = m,
            Task::Mem((addr, value)) => {
                for a in mask.mem_space(*addr) {
                    computer.run(a, *value);
                }
            }
        }
    }
    computer.sum()
}

fn main() -> io::Result<()>{
    let input = parse_file("input.txt")?;
    let p1 = part1(&input);
    println!("Part 1: {}", p1);
    let p2 = part2(&input);
    println!("Part 2: {}", p2);
    Ok(())
}
