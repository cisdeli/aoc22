/*
 * RUCKSACK REORGANIZATION
 * Input: items currently in each rucksack
 * - Item type is case sensitive
 * - First half of characters represent the items on the first compartment
 * - Second half represent the second compartment
 * - Priorities:
 *  - a throught z has priorities 1-26
 *  - A-Z = 27-52
 * Output: find the sum of the priority of the item type that appers
 * in both compartments of each rucksack.
 */

use std::vec;

fn char_to_int(c: char) -> Option<u32> {
    match c {
        'a'..='z' => Some(c as u32 - 'a' as u32 + 1),
        'A'..='Z' => Some(c as u32 - 'A' as u32 + 27),
        _ => None,
    }
}

fn sum_priorities(rucksack: String) -> u32 {
    if rucksack.len() % 2 != 0 {
        panic!("Can't split rucksack into two compartments of the same size");
    }
    let mut sum = 0;
    let (compartment_one, compartment_two) = rucksack.split_at(rucksack.len() / 2);
    let mut chars_in_common: Vec<char> = vec![];
    for char_in_c1 in compartment_one.chars() {
        if compartment_two.contains(char_in_c1) && !chars_in_common.contains(&char_in_c1) {
            chars_in_common.push(char_in_c1);
        }
    }
    for c in chars_in_common {
        sum += char_to_int(c).unwrap_or_else(|| {
            println!("Invalid charachter: {}", c);
            0
        });
    }
    return sum;
}

pub fn solution(input: String) {
    let mut sum = 0;
    for line in input.lines() {
        sum += sum_priorities(line.to_string());
    }
    println!("Sum for part 1: {}", sum);
}
