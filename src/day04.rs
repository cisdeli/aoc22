/*
 * CAMP CLEANUP
 * Input: Section assignments for each pair of elves
 * Output: how many assignments pairs does one range fully contain the other
 */

fn parse_pair(pair: &str) -> (u32, u32) {
    let mut parts = pair.split('-');
    let start = parts.next().unwrap().parse().unwrap();
    let end = parts.next().unwrap().parse().unwrap();
    return (start, end);
}

fn check_pair(pair_one: &str, pair_two: &str) -> bool {
    let (start1, end1) = parse_pair(pair_one);
    let (start2, end2) = parse_pair(pair_two);

    let case1 = start1 <= start2 && end1 >= end2;
    let case2 = start2 <= start1 && end2 >= end1;
    return case1 || case2;
}

pub fn solution(input: String) {
    // let input = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8";
    let mut count = 0;
    for line in input.lines() {
        let mut pairs = line.split(',');
        if let (Some(pair_one), Some(pair_two)) = (pairs.next(), pairs.next()) {
            if check_pair(pair_one, pair_two) {
                count += 1;
            }
        } else {
            println!("Invalid pair format: {}", line);
        }
    }
    
    println!("Count: {}", count);
}
