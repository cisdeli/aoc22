/*
 * TUNING TROUBLE
 * To-do: try to optimize this solution
*/

use std::collections::HashSet;

fn check_dup(s: &str) -> bool {
    let mut seen_chars = HashSet::new();
    for c in s.chars() {
        if !seen_chars.insert(c) {
            return true;
        }
    }
    return false;
}

fn detect_first_sop(input: String, pt: i32) {
    let mut step: usize = 0;
    if pt == 1 {
        step = 4;
    } else if pt == 2 {
        step = 14;
    }
    for i in 0..input.len() {
        let sequence = &input[i..i + step];
        if check_dup(sequence) == false {
            println!("{} {}", sequence, i + step);
            break;
        }
    }
}

pub fn solution(input: String) {
    detect_first_sop(input, 2);
}
