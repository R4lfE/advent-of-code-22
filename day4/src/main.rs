use std::{error::Error, fs, cmp::{max, min}};

fn get_ans(input: &String, eval: fn(Vec<u32>) -> bool) -> u32 {
    input.lines()
        .filter(|line| {
            let bounds: Vec<u32> = line
                .split(",")
                .map(|elf| elf
                    .split("-")
                    .map(|bound| bound
                        .parse()
                        .unwrap()))
                .flatten()
                .collect();
            eval(bounds)
        })
        .count()
        .try_into()
        .unwrap()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;

    let part1 = get_ans(&input, |bounds| {
        (bounds[0] <= bounds[2] && bounds[1] >= bounds[3])
        || (bounds[0] >= bounds[2] && bounds[1] <= bounds[3])
    });
    println!("part 1: {}", part1);

    let part2 = get_ans(&input, |bounds| {
        max(bounds[0], bounds[2]) <= min(bounds[1], bounds[3])
    });
    println!("part 2: {}", part2);

    Ok(())
}
