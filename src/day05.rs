/*
 * SUPPLY STACKS
 */

use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

fn fill_empty_crates(lines: Vec<&str>, max_height: usize) -> String {
    let mut crates_str = String::new();

    for i in 0..max_height {
        let line = lines[i];
        let aux = &format!("{}{}\n", line.replace("    ", "[-]"), "");
        crates_str.push_str(&aux.replace(" ", ""));
    }

    crates_str
}

fn parse_input(input: String) -> (HashMap<i32, Vec<char>>, Vec<(char, i32, i32)>) {
    let mut crates: HashMap<i32, Vec<char>> = HashMap::new();
    let mut moves: Vec<(char, i32, i32)> = Vec::new();
    let lines: Vec<&str> = input.lines().collect();

    let max_height = lines
        .iter()
        .take_while(|line| line.trim().starts_with('['))
        .count();

    // Crates parsing
    let crates_str = fill_empty_crates(lines.clone(), max_height);
    for line in crates_str.lines() {
        let chars = line.chars().filter(|c| *c != '[' && *c != ']');
        for (i, c) in chars.enumerate() {
            crates
                .entry((i + 1) as i32)
                .or_insert_with(Vec::new)
                .push(c);
        }
    }

    // Moves parsing
    for line in lines[max_height..]
        .iter()
        .filter(|line| line.starts_with("move"))
    {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let crate_num = parts[1].chars().next().unwrap();
        let from = parts[3].parse::<i32>().unwrap();
        let to = parts[5].parse::<i32>().unwrap();
        moves.push((crate_num, from, to));
    }

    println!("crates: {:?}", crates);
    println!("moves: {:?}", moves);
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

            let (crates, moves) = parse_input(input);
            // println!("{:?}", crates);
            // println!("{:?}", moves);
        }
        Err(error) => {
            println!("Error: {}", error);
        }
    }
}
