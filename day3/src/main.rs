#![feature(iter_array_chunks)]
use std::{error::Error, fs, collections::HashSet};

fn char_to_alpha_order(char: char) -> u32 {
    if char.is_ascii_lowercase() {
        char as u32 - 96
    } else {
        char as u32 - 38
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;

    let part1: u32 = input
        .lines()
        .map(|sack| {
            let map: HashSet<char> = sack[..sack.len()/2].chars().collect();
            
            for s in sack[sack.len()/2..].chars() {
                if map.contains(&s) {
                    return char_to_alpha_order(s);
                }
            }
            0
        })
        .sum();

    println!("{:?}", part1);

    let part2: u32 = input
        .lines()
        .map(|sack| sack.chars().collect::<HashSet<char>>())
        .array_chunks::<3>()
        .map(|[first, second, third]| {
            for s in first.intersection(&second) {
                if third.contains(s) {
                    return char_to_alpha_order(*s);
                }
            }
            0
        })
        .sum();

    println!("{:?}", part2);

    Ok(())
}
