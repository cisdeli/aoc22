/*
 * CALORIE COUNTING
 * Input: amount of calories each elf is carrying.
 *  - Each line represents one item
 *  - Balnk line separetes the invetory of each elf
 * Output 1: max quantity of calories that an elf in carrying
 * Output 2: sum of the top 3 elves carrying the max quantity of calories
*/

fn max_calories(input: String, choice: String) -> u32 {
    let mut max_vec: Vec<u32> = Vec::new();
    let mut curr_max: u32 = 0;

    for line in input.lines() {
        if line.trim().is_empty() {
            max_vec.push(curr_max);
            max_vec.sort_by(|a, b| b.cmp(a)); // sorting in descending order
            if choice == "Part One" {
                max_vec.truncate(1);
            } else if choice == "Part Two" {
                max_vec.truncate(3);
            }
            curr_max = 0;
        } else {
            match line.trim().parse::<u32>() {
                Ok(num) => curr_max += num,
                Err(_) => println!("Failed to parse line as u32: {}", line),
            }
        }
    }
    return max_vec.iter().sum();
}

pub fn solution(input: String) {
    println!("Part 1: {}", max_calories(input.clone(), format!("Part One")));
    println!("Part 2: {}", max_calories(input.clone(), format!("Part Two")));
}
