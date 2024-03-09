/*
 * SUPPLY STACKS
 * Input:
 * Output:
 */

use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

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

pub fn solution(_input: String) {
    let file = File::options().read(true).open("src/input.txt");

    match file {
        Ok(file) => {
            let mut buff_reader = BufReader::new(file);
            let mut input = String::new();
            let _ = buff_reader.read_to_string(&mut input);
            println!("{}", input);
            
            // let (crates, moves) = parse_input(input);
            // println!("{:?}", crates);
            // println!("{:?}", moves);
        },
        Err(error) => {
            println!("Error: {}", error);
        }
    }

    // parse_input();
}
