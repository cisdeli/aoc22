/*
 * RUCKSACK REORGANIZATION
 * Input: items currently in each rucksack
 * - Item type is case sensitive
 * - First half of characters represent the items on the first compartment
 * - Second half represent the second compartment
 * - Priorities:
 *  - a throught z has priorities 1-26
 *  - A-Z = 27-52
 * Output: find the sum of the priority of the item type that appears
 * in both compartments of each rucksack.
 * Output for part 2: find the sum of the priorities of the item type that appears
 * on the group of 3 elves (find the letter that appers on groups of 3 lines)
 */

use std::collections::HashSet;

fn char_to_int(c: char) -> Option<u32> {
    match c {
        'a'..='z' => Some(c as u32 - 'a' as u32 + 1),
        'A'..='Z' => Some(c as u32 - 'A' as u32 + 27),
        _ => None,
    }
}

fn sum_priorities_part_one(rucksack: String) -> u32 {
    if rucksack.len() % 2 != 0 {
        panic!("Can't split rucksack into two compartments of the same size");
    }
    let mut sum = 0;
    let (compartment_one, compartment_two) = rucksack.split_at(rucksack.len() / 2);
    let mut chars_in_common = HashSet::new();

    for char_in_c1 in compartment_one.chars() {
        if compartment_two.contains(char_in_c1) && chars_in_common.insert(char_in_c1) {
            sum += char_to_int(char_in_c1).unwrap_or_else(|| {
                println!("Invalid character: {}", char_in_c1);
                0
            });
        }
    }
    return sum;
}

fn sum_priorities_part_two(group_of_rucksacks: &Vec<String>) -> u32 {
    let mut sum = 0;
    // By converting the strings to hashsets with exclude any dupes
    // Then we can use the function intersect() to check the char in common
    let set1: HashSet<_> = group_of_rucksacks[0].chars().collect();
    let set2: HashSet<_> = group_of_rucksacks[1].chars().collect();
    let set3: HashSet<_> = group_of_rucksacks[2].chars().collect();
    // Really cool way of intersecting multiple hashsets
    // see: https://www.reddit.com/r/rust/comments/5v35l6/intersection_of_more_than_two_sets/
    // for more information
    let chars_in_common = set1.iter().filter(|k| set2.contains(k)).filter(|k| set3.contains(k));
    for c in chars_in_common {
        sum += char_to_int(*c).unwrap_or_else(|| {
                println!("Invalid character: {}", c);
                0
            });
    }
    return sum;
}

pub fn solution(input: String) {
    let mut sum = 0;
    let mut sum_pt2 = 0;
    let mut i = 0;
    let mut group_of_3: Vec<String> = vec![];
    for line in input.lines() {
        sum += sum_priorities_part_one(line.to_string());

        group_of_3.push(line.to_string());
        i += 1;
        if i == 3 {
            sum_pt2 += sum_priorities_part_two(&group_of_3);
            group_of_3.clear();
            i = 0
        }
    }
    println!("Sum for part 1: {}", sum);
    println!("Sum for part 2: {}", sum_pt2);
}
