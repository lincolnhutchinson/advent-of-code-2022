use std::fs;

#[derive(Debug, Copy, Clone)]
enum Choices {
    Rock,
    Paper,
    Scissors,
}

impl Choices {
    fn all() -> [Self; 3] {
        [Self::Rock, Self::Paper, Self::Scissors]
    }

    fn score(&self) -> u32 {
        match *self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn play_against(&self, opponent: &Self) -> RoundResult {
        let my_score = self.score();
        let opponent_score = opponent.score();

        if my_score == opponent_score {
            RoundResult::Draw
        } else if opponent_score + 1 == my_score || my_score + 2 == opponent_score {
            RoundResult::Win
        } else {
            RoundResult::Lose
        }
    }

    fn get_my_play_for_result(opponent: &Self, desired_result: RoundResult) -> Self {
        *Self::all()
            .iter()
            .find(|choice| choice.play_against(opponent) == desired_result)
            .unwrap()
    }

    fn from_abc(c: char) -> Self {
        match c {
            'A' => Self::Rock,
            'B' => Self::Paper,
            'C' => Self::Scissors,
            _ => panic!("Unexpected char {c} is not A, B, or C."),
        }
    }

    fn from_xyz(c: char) -> Self {
        match c {
            'X' => Self::Rock,
            'Y' => Self::Paper,
            'Z' => Self::Scissors,
            _ => panic!("Unexpected char {c} is not X,Y or Z."),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum RoundResult {
    Win,
    Lose,
    Draw,
}

impl RoundResult {
    fn score(&self) -> u32 {
        match *self {
            Self::Win => 6,
            Self::Draw => 3,
            Self::Lose => 0,
        }
    }

    fn from_xyz(c: char) -> Self {
        match c {
            'X' => Self::Lose,
            'Y' => Self::Draw,
            'Z' => Self::Win,
            _ => panic!(),
        }
    }
}

fn main() {
    println!("Hello Day2");

    let raw_data = fs::read_to_string("src/data/day2.txt").unwrap();

    let result = score_strategy(&raw_data);

    println!("The score of the given strategy is {result}");
}

fn score_strategy(strat: &str) -> u32 {
    strat
        .trim()
        .split("\n")
        .map(|s| {
            let trimmed = s.trim();
            println!("Parsing score for string {trimmed}");
            let opponent_char = trimmed.chars().next().unwrap();
            let result_char = trimmed.chars().last().unwrap();

            let opponent_play = Choices::from_abc(opponent_char);
            let result = RoundResult::from_xyz(result_char);
            let my_play = Choices::get_my_play_for_result(&opponent_play, result);

            let total_score = my_play.score() + result.score();

            println!("I played {my_play:?} and the opponent played {opponent_play:?}");
            println!("I got a result of {result:?} and a total score of {total_score}");

            total_score
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_strat() {
        const DATA: &str = "A Y
                            B X
                            C Z";
        const EXPECTED: u32 = 12;
        let actual = score_strategy(DATA);

        assert_eq!(actual, EXPECTED);
    }

    #[test]
    fn test_different_strat() {
        const DATA: &str = "A Y
                            B X
                            A Z
                            C Z";
        const EXPECTED: u32 = 20;
        let actual = score_strategy(DATA);

        assert_eq!(actual, EXPECTED);
    }
}
