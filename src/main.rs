mod day01;
mod day02;
mod day03;
mod day04;

use dotenv::dotenv;
use reqwest;
use std::env;
use std::error::Error;

// Function to get input from Advent of Code
async fn get_input(day: u32) -> Result<String, Box<dyn Error>> {
    let url = format!("https://adventofcode.com/2022/day/{}/input", day);
    let cookie = env::var("SESSION_COOKIE").expect("SESSION COOKIE not set");
    let response = reqwest::Client::new()
        .get(&url)
        .header("Cookie", cookie)
        .send()
        .await?;
    let input = response.text().await?;
    return Ok(input.trim().to_string());
}

// I still don't understand everything from this function but oh well
type SolutionFunction = fn(String);
async fn process_solution(day: u32) {
    let solutions: Vec<SolutionFunction> = vec![
        day01::solution,
        day02::solution,
        day03::solution,
        day04::solution,
    ];

   match solutions.get(day as usize - 1) {
       Some(solution) => {
           let input = get_input(day)
               .await
               .expect(&format!("Error fetching input for day {}", day)); // the & is used for
                                                                          // string concatenation
           solution(input);
       }
       None => {
           println!("Invalid day specified");
       }
   }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Loading .env file
    dotenv().ok();

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: cargo watch -x 'run <day_number>' --poll");
        std::process::exit(1);
    }

    let day: u32 = args[1].parse().expect("Error while parsing args");
    process_solution(day).await;

    return Ok(());
}
