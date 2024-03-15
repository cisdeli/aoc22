/*
 * ROCK PAPER SCISSORS
 * Input: Encrypted strategy guide
 *  - First column is what the first is going to play.
 *  - A: Rock
 *  - B: Paper
 *  - C: Scissors
 *  - Second column is assumed to be what I should play.
 *  - X: Rock
 *  - Y: Paper
 *  - Z: scissors.
 *  ** Part 2**
 *  - Second column is the result of the match.
 *  - X: lose
 *  - Y: draw
 *  - Z: win
 * Winner of the tournament is the player with highest score
 * The shape played affect your score:
 * - Rock: 1
 * - Paper: 2
 * - Scissors: 3
 * - Plus 0 if lost and 3 if it was a draw and 6 if won.
 * Output: calculate your score following the strategy guide.
 */

#[derive(Debug, PartialEq, Eq)]
enum Play {
    Rock,
    Paper,
    Scissors,
}
#[derive(Debug, PartialEq, Eq)]
enum Result {
    Lost,
    Draw,
    Win,
}
#[derive(Debug)]
struct Game {
    first: Play,
    second: Play,
    second_pt2: Result,
}

// Game logic
impl Game {
    // Constructor
    fn new(first_col: char, second_col: char, second_col_pt2: char) -> Option<Self> {
        let first = match first_col {
            'A' => Some(Play::Rock),
            'B' => Some(Play::Paper),
            'C' => Some(Play::Scissors),
            _ => None,
        };
        let second = match second_col {
            'X' => Some(Play::Rock),
            'Y' => Some(Play::Paper),
            'Z' => Some(Play::Scissors),
            _ => None,
        };
        let second_pt2 = match second_col_pt2 {
            'X' => Some(Result::Lost),
            'Y' => Some(Result::Draw),
            'Z' => Some(Result::Win),
            _ => None,
        };
        first.and_then(|a| {
            second.and_then(|b| {
                second_pt2.map(|c| Game {
                    first: a,
                    second: b,
                    second_pt2: c,
                })
            })
        })
    }

    fn game_logic(&self) -> Result {
        match (&self.first, &self.second) {
            (Play::Rock, Play::Scissors)
            | (Play::Paper, Play::Rock)
            | (Play::Scissors, Play::Paper) => Result::Lost,

            (Play::Rock, Play::Paper)
            | (Play::Paper, Play::Scissors)
            | (Play::Scissors, Play::Rock) => Result::Win,

            _ => Result::Draw,
        }
    }
    fn game_logic_2(&self) -> Play {
        match (&self.first, &self.second_pt2) {
            (Play::Rock, Result::Lost) => Play::Scissors,
            (Play::Scissors, Result::Lost) => Play::Paper,
            (Play::Paper, Result::Lost) => Play::Rock,

            (Play::Rock, Result::Win) => Play::Paper,
            (Play::Scissors, Result::Win) => Play::Rock,
            (Play::Paper, Result::Win) => Play::Scissors,

            (Play::Rock, Result::Draw) => Play::Rock,
            (Play::Scissors, Result::Draw) => Play::Scissors,
            (Play::Paper, Result::Draw) => Play::Paper,
        }
    }

    fn calculate_score(&self) -> (u32, u32) {
        let result = self.game_logic();
        let result2 = self.game_logic_2();

        let mut score = match self.second {
            Play::Rock => 1,
            Play::Paper => 2,
            Play::Scissors => 3,
        };
        let mut score2 = match result2 {
            Play::Rock => 1,
            Play::Paper => 2,
            Play::Scissors => 3,
        };

        score += match result {
            Result::Lost => 0,
            Result::Draw => 3,
            Result::Win => 6,
        };
        score2 += match self.second_pt2 {
            Result::Lost => 0,
            Result::Draw => 3,
            Result::Win => 6,
        };

        (score, score2)
    }
}

pub fn solution(input: String) {
    let mut score_pt1 = 0;
    let mut score_pt2 = 0;
    for line in input.lines() {
        let plays: Vec<&str> = line.split_whitespace().collect();
        let (first_col, second_col) = (
            plays[0].chars().next().unwrap_or(' '), // Default to space if None
            plays[1].chars().next().unwrap_or(' '), // Default to space if None
        );
        let game = Game::new(first_col, second_col, second_col).unwrap_or_else(|| {
            panic!("Invalid plays provided");
        });

        let (aux_score_pt1, aux_score_pt2) = game.calculate_score();
        score_pt1 += aux_score_pt1;
        score_pt2 += aux_score_pt2;
    }
    println!("Score part 1: {}", score_pt1);
    println!("Score part 2: {}", score_pt2);
}
