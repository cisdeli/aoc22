/*
 * CAMP CLEANUP
 * Input: Section assignments for each pair of elves
 * Output: how many assignments pairs does one range fully contain the other
 * Output 2: how many assignments pairs does one range overlaps the other
 */

fn parse_pair(pair: &str) -> (u32, u32) {
    let mut parts = pair.split('-');
    let start = parts.next().unwrap().parse().unwrap();
    let end = parts.next().unwrap().parse().unwrap();
    return (start, end);
}

fn check_pairs(pair_one: &str, pair_two: &str) -> (bool, bool) {
    let (start1, end1) = parse_pair(pair_one);
    let (start2, end2) = parse_pair(pair_two);

    // Checking if pair contains the other
    let case1 = start1 <= start2 && end1 >= end2;
    let case2 = start2 <= start1 && end2 >= end1;
    let contains = case1 || case2;

    // Checking if overlaps
    let overlap1 = start1 <= start2 && end1 >= start2 && end1 <= end2;
    let overlap2 = start2 <= start1 && end2 >= start1 && end2 <= end1;
    let overlap3 = start1 >= start2 && end1 <= end2;
    let overlap4 = start2 >= start1 && end2 <= end1;
    let overlaps = overlap1 || overlap2 || overlap3 || overlap4;
    
    return (contains, overlaps);
}

pub fn solution(input: String) {
    let mut contains_count = 0;
    let mut overlaps_count = 0;
    for line in input.lines() {
        let mut pairs = line.split(',');
        if let (Some(pair_one), Some(pair_two)) = (pairs.next(), pairs.next()) {
            let (contains, overlaps) = check_pairs(pair_one, pair_two);
            if contains {
                contains_count += 1;
            }
            if overlaps {
                overlaps_count += 1;
            }
        } else {
            println!("Invalid pair format: {}", line);
        }
    }
    
    println!("Contains Count: {}", contains_count);
    println!("Overlaps Count: {}", overlaps_count);
}
