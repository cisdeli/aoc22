/*
 * TUNING TROUBLE
 * To-do: try to optimize this solution -> Done
*/

use std::collections::HashSet;
use std::time::Instant;

// Sliding window solution
fn sliding_window_check_dup(string: String, pt: i32) -> usize {
    let input: Vec<char> = string.chars().collect();
    let marker_size: usize = if pt == 1 { 4 } else { 14 };
    let mut occurrences = [0u16; 256]; // this doesn't need to be 256 long. It's an array of zeroes that represent the utf encoding of the letters e.g. m = 109
    let mut different_letters = 0;
    input
        .iter()
        .enumerate()
        .find_map(|(n, &i)| {
            if n >= marker_size {
                occurrences[input[n - marker_size] as usize] -= 1;
                if occurrences[input[n - marker_size] as usize] == 0 {
                    different_letters -= 1; // the letter MARKER_SIZE ago is no longer present in the sliding window so decrement different_letters
                }
            }
            if occurrences[i as usize] == 0 {
                different_letters += 1; // because it was not present in occurrences before, this is a new letter so increment different_letters
            }
            occurrences[i as usize] += 1;
            if different_letters == marker_size {
                Some(n + 1) // There are MARKER_SIZE different letters so the answer has been found
            } else {
                None
            }
        })
        .unwrap()
}

// Solution using the bytes of the input string
fn bytes_check_dup(s: &[u8]) -> bool {
    let mut seen_bytes = HashSet::new();
    for byte in s {
        seen_bytes.insert(byte);
    }
    seen_bytes.len() == s.len()
}

fn bytes_detection(input: String, pt: i32) -> usize {
    let step: usize = if pt == 1 { 4 } else { 14 };
    let bytes = input.as_bytes();
    let mut i = 0;
    loop {
        if bytes_check_dup(&bytes[i..i + step]) {
            return i + step;
        }
        i += 1;
        if i == bytes.len() {
            break;
        }
    }
    panic!("Solution not found");
}
//

// My initial solution
fn check_dup(s: &str) -> bool {
    let mut seen_chars = HashSet::new();
    for c in s.chars() {
        if !seen_chars.insert(c) {
            return true;
        }
    }
    return false;
}

fn detect_first_sop(input: String, pt: i32) -> usize {
    let step = if pt == 1 { 4 } else { 14 };
    for i in 0..input.len() {
        let sequence = &input[i..i + step];
        if check_dup(sequence) == false {
            return i + step;
        }
    }
    panic!("Solution not found");
}
//

pub fn solution(input: String) {
    let part: i32 = 2;

    // Benchmarking my solution
    let now = Instant::now();
    let res = detect_first_sop(input.clone(), part);
    let elapsed = now.elapsed();
    println!("My solution: {}, time spent: {:.2?}", res, elapsed);

    // Benchmarking bytes solution
    let now = Instant::now();
    let res = bytes_detection(input.clone(), part);
    let elapsed = now.elapsed();
    println!("Bytes solution: {}, time spent: {:.2?}", res, elapsed);

    // Benchmarking sliding window solution
    let now = Instant::now();
    let res = sliding_window_check_dup(input.clone(), part);
    let elapsed = now.elapsed();
    println!("Sliding Window solution: {}, time spent: {:.2?}", res, elapsed);
}
