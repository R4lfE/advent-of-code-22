use std::{error::Error, fs, cmp::Ordering::{Greater, Equal, Less}, str::FromStr};

#[derive(Clone, Copy)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl PartialEq for Move {
    fn eq(&self, other: &Self) -> bool {
        *self as u8 == *other as u8
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self == &Move::Rock && other == &Move::Scissors {
            Some(Greater)
        } else if self == &Move::Scissors && other == &Move::Rock {
            Some(Less)
        } else {
            Some((*self as u8).cmp(&(*other as u8)))
        }
    }

    fn lt(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(other), Some(Less))
    }

    fn le(&self, other: &Self) -> bool {
        // Pattern `Some(Less | Eq)` optimizes worse than negating `None | Some(Greater)`.
        // FIXME: The root cause was fixed upstream in LLVM with:
        // https://github.com/llvm/llvm-project/commit/9bad7de9a3fb844f1ca2965f35d0c2a3d1e11775
        // Revert this workaround once support for LLVM 12 gets dropped.
        !matches!(self.partial_cmp(other), None | Some(Greater))
    }

    fn gt(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(other), Some(Greater))
    }

    fn ge(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(other), Some(Greater | Equal))
    }
}

impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Move::Rock),
            "B" | "Y" => Ok(Move::Paper),
            "C" | "Z" => Ok(Move::Scissors),
            _ => Err("Invalid choice.".to_string())
        }
    }
}

enum MatchResult {
    Win,
    Draw,
    Lose
}

impl FromStr for MatchResult {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(MatchResult::Lose),
            "Y" => Ok(MatchResult::Draw),
            "Z" => Ok(MatchResult::Win),
            _ => Err("Invalid result.".to_string())
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = fs::read_to_string("input.txt")?;

    // res of 1
    let result: u32 = file
        .lines()
        .map(|line| {
            let round: Vec<Move> = line
                .split(" ")
                .map(|s| s.parse::<Move>().unwrap())
                .collect();
            match round[0].partial_cmp(&round[1]) {
                Some(Less) => 6 + round[1] as u32,
                Some(Equal) => 3 + round[1] as u32,
                Some(Greater) => round[1] as u32,
                None => panic!("???"),
            }
        })
        .sum();

    println!("Part 1: {}", result);

    let result: u32 = file
        .lines()
        .map(|line| {
            let round: Vec<&str> = line.split(" ").collect();
            let opponent = round[0].parse::<Move>().unwrap();
            match round[1].parse::<MatchResult>().unwrap() {
                MatchResult::Win => (opponent as u32 % 3) + 1 + 6,
                MatchResult::Draw => opponent as u32 + 3,
                MatchResult::Lose => 3 - (4 - opponent as u32) % 3,
            }
        })
        .sum();

    println!("Part 2: {}", result);

    Ok(())
}
