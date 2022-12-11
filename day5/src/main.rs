use std::{error::Error, fs};

type Stack<T> = Vec<T>;
type Cargo<T> = Vec<Stack<T>>;

fn build_initial_state(input: Vec<&str>) -> Cargo<char> {
    let mut i = 0;
    let mut iter = input.iter();
    while let Some(&l) = iter.next() {
        if l == "" {
            i -= 1;
            break;
        }
        i += 1;
    }
    
    let mut state = Cargo::<char>::new();
    for _j in 0..input[i].split_whitespace().count() {
        state.push(Stack::<char>::new());
    }

    let stacks = state.len();
    for l in (0..i).rev() {
        println!("{}", input[l]);
        for j in 0..stacks {
            let char = input[l][4*j..4*(j+1)].chars().nth(1).unwrap();
            if char != ' ' {
                state[j].push(char);
            }
        }
    }

    state
}

fn move_crates(state: &mut Cargo<char>, str: &str) {
    let mov: Vec<usize> = str
        .split_ascii_whitespace()
        .filter(|&s| s.parse::<u32>().is_ok())
        .map(|s| s.parse::<u32>().unwrap() as usize)
        .collect();

    for _i in 0..mov[0] {
        if !state[mov[1]-1].is_empty() {
            let move_crate = state[mov[1]-1].pop().unwrap();
            state[mov[2]-1].push(move_crate);
        }
    }
}

fn move_crates_2(state: &mut Cargo<char>, str: &str) {
    let mov: Vec<usize> = str
        .split_ascii_whitespace()
        .filter(|&s| s.parse::<u32>().is_ok())
        .map(|s| s.parse::<u32>().unwrap() as usize)
        .collect();

    let mut temp: Stack<char> = Stack::new();
    for _i in 0..mov[0] {
        if !state[mov[1]-1].is_empty() {
            let move_crate = state[mov[1]-1].pop().unwrap();
            temp.push(move_crate);
        }
    }
    while let Some(move_crate) = temp.pop() {
        state[mov[2]-1].push(move_crate);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    let mut state = build_initial_state(input.lines().collect());
    for mov in input.lines() {
        if mov.chars().count() > 0 && mov.chars().nth(0).unwrap() == 'm' {
            move_crates_2(&mut state, mov);
        }
    }
    println!("{:?}", state);
    Ok(())
}
