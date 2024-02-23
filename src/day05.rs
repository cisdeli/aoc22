/*
 * SUPPLY STACKS
 * Input:
 * Output:
 */

use std::collections::HashMap;

fn parse_input(input: String) -> (HashMap<char, i32>, Vec<(char, i32, i32)>) {
    let mut crates: HashMap<char, i32> = HashMap::new();
    let mut moves: Vec<(char, i32, i32)> = Vec::new();
    let lines: Vec<&str> = input.lines().collect();

    let num_crates = lines
        .iter()
        .take_while(|line| line.trim().starts_with('['))
        .count();

    // Parse the initial positions of the crates
    for i in 0..num_crates {
        let line = lines[i];
        let position = i + 1;
        for c in line.chars().filter(|c| c.is_alphabetic()) {
            crates.insert(c, position.try_into().unwrap());
        }
    }

    for line in lines[num_crates..]
        .iter()
        .filter(|line| line.starts_with("move"))
    {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let crate_num = parts[1].chars().next().unwrap();
        let from = parts[3].parse::<i32>().unwrap();
        let to = parts[5].parse::<i32>().unwrap();
        moves.push((crate_num, from, to));
    }

    // println!("crates: {:?}", crates);
    // println!("moves: {:?}", moves);
    // println!("input: {}", input);
    return (crates, moves);
}

fn solve(input: String) {
    let (mut crates, moves) = parse_input(input);
    let mut stacks: HashMap<i32, Vec<char>> = HashMap::new();

    // Initialize the stacks with the initial positions of the crates
    for (&c, &position) in &crates {
        stacks.entry(position).or_insert_with(Vec::new).push(c);
    }

    for (c, from, to) in moves {
        if let Some(position) = crates.get_mut(&c) {
            if *position == from {
                *position = to;
                // Remove the crate from the old position and add it to the new position
                stacks.get_mut(&from).unwrap().retain(|&x| x != c);
                stacks.entry(to).or_insert_with(Vec::new).push(c);
            }
        }
    }

    // Print the crate at the top of each stack
    for (position, stack) in &stacks {
        if let Some(c) = stack.last() {
            println!("Position {}: Crate {}", position, c);
        }
    }
}

pub fn solution(input: String) {
    solve(input);
}
