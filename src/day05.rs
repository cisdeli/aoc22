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

fn parse_input(input: String) -> (HashMap<i32, Vec<char>>, Vec<(i32, i32, i32)>) {
    let mut crates: HashMap<i32, Vec<char>> = HashMap::new();
    let mut moves: Vec<(i32, i32, i32)> = Vec::new();
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
            if c.is_alphabetic() {
                // not pushing empty crates
                crates
                    .entry((i + 1) as i32)
                    .or_insert_with(Vec::new)
                    .push(c);
            }
        }
    }

    // Moves parsing
    for line in lines[max_height..]
        .iter()
        .filter(|line| line.starts_with("move"))
    {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let crate_num = parts[1].parse::<i32>().unwrap();
        let from = parts[3].parse::<i32>().unwrap();
        let to = parts[5].parse::<i32>().unwrap();
        moves.push((crate_num, from, to));
    }

    println!("crates: {:?}", crates);
    println!("moves: {:?}", moves);
    (crates, moves)
}

fn solve(mut crates: HashMap<i32, Vec<char>>, moves: Vec<(i32, i32, i32)>) {
    for (_, vec) in crates.iter_mut() {
        vec.reverse();
    }
    for mv in moves.iter() {
        let crate_qtt = mv.0;
        let from = mv.1;
        let to = mv.2;

        let from_stack = crates.get_mut(&from).unwrap();

        let mut crates_aux = Vec::new();
        for _ in 0..crate_qtt {
            if let Some(c) = from_stack.pop() {
                crates_aux.push(c);
            }
        }
        crates.get_mut(&to).unwrap().extend(crates_aux);
        println!("moved");
        println!("{:?}", crates);
    }

    let mut sorted_crates: Vec<_> = crates.into_iter().collect();
    sorted_crates.sort_by_key(|&(key, _)| key);
    for (key, vec) in sorted_crates {
        if let Some(last_char) = vec.last() {
            println!("Key {}: {}", key, last_char);
        } else {
            println!("Key {}: Empty vector", key);
        }
    }
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
            solve(crates, moves);
        }
        Err(error) => {
            println!("Error: {}", error);
        }
    }

    // let (crates, moves) = parse_input(input);
    // solve(crates, moves);
}
