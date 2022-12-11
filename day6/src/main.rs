use std::{error::Error, fs, collections::HashSet};

fn ans(input: &String, window_size: usize) -> usize {
    for window in input
        .as_bytes()
        .windows(window_size)
        .enumerate() {
        if HashSet::<&u8>::from_iter(window.1.iter()).len() == window_size {
            return window.0 + window_size;
        }
    }
    0
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;

    let part_1 = ans(&input, 4);
    let part_2 = ans(&input, 14);
    println!("part 1: {}, part 2: {}", part_1, part_2);

    Ok(())
}
