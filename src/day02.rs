/*
 * ROCK PAPER SCISSORS
 * Input: Encrypted strategy guide
 *  - First column is what the opponent is going to play.
 *  - A: Rock
 *  - B: Paper
 *  - C: Scissors
 *  - Second column is assumed to be what I should play.
 *  - X: Rock
 *  - Y: Paper
 *  - Z: scissors.
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
#[derive(Debug)]
struct Game {
    opponent: Play,
    me: Play,
}

impl Game {
    fn new(opp_play: char, my_play: char) -> Option<Self> {
        let opponent = match opp_play {
            'A' => Some(Play::Rock),
            'B' => Some(Play::Paper),
            'C' => Some(Play::Scissors),
            _ => None,
        };
        let me = match my_play {
            'X' => Some(Play::Rock),
            'Y' => Some(Play::Paper),
            'Z' => Some(Play::Scissors),
            _ => None,
        };
        opponent.and_then(|a| me.map(|b| Game { opponent: a, me: b }))
    }

    fn game_logic(&self) -> &str {
        match (&self.opponent, &self.me) {
            (Play::Rock, Play::Scissors)
            | (Play::Paper, Play::Rock)
            | (Play::Scissors, Play::Paper) => "opp",

            (Play::Rock, Play::Paper)
            | (Play::Paper, Play::Scissors)
            | (Play::Scissors, Play::Rock) => "me",

            _ => "draw",
        }
    }

    fn calculate_score(&self) -> u32 {
        let result = self.game_logic();

        let mut score = match self.me {
            Play::Rock => 1,
            Play::Paper => 2,
            Play::Scissors => 3,
        };

        score += match result {
            "opp" => 0,
            "me" => 6,
            "draw" => 3,
            _ => 0, // Handle unexpected results (you might want to log an error or panic here)
        };
        return score;
    } 
}

pub fn solution(input: String) {
    let mut score = 0;
    for line in input.lines() {
        let plays: Vec<&str> = line.split_whitespace().collect();
        let (opp_play, my_play) = (
            plays[0].chars().next().unwrap_or(' '),  // Default to space if None
            plays[1].chars().next().unwrap_or(' '),  // Default to space if None
        );
        let game = Game::new(opp_play, my_play).unwrap_or_else(|| {
            panic!("Invalid plays provided");
        });
        score += game.calculate_score();
    }
    println!("Score: {}", score);
}
