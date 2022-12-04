use std::{fs, error::Error, collections::BinaryHeap};

fn get_most_calories(input: &Vec<i32>) -> i32 {
    let mut sum = 0;
    let mut max = 0;

    for val in input {
        if *val == -1 {
            if sum > max {
                max = sum;
            }
            sum = 0;
        } else {
            sum += val;
        }
    }

    max
}

fn get_3_most_calories(input: &Vec<i32>) -> i32 {
    let mut calorie_queue: BinaryHeap<i32> = BinaryHeap::new();
    let mut sum = 0;

    for val in input {
        if *val == -1 {
            calorie_queue.push(sum);
            sum = 0;
        } else {
            sum += val;
        }
    }

    sum = 0;
    for _i in 0..3 {
        if let Some(val) = calorie_queue.pop() {
            sum += val;
        }
    }

    sum
}

fn main() -> Result<(), Box<dyn Error>> {
    let input: Vec<i32> = fs::read_to_string("input.txt")?
        .split("\r\n").map(|s| {
            if s == "" {
                -1
            } else {
                s.parse::<i32>().unwrap()
            }
        })
        .collect();
    
    println!("{}", get_most_calories(&input));
    println!("{}", get_3_most_calories(&input));
    Ok(())
}
