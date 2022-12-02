use std::fs::{read, File};
use std::io::{prelude::*, BufReader};
use Choice::*;
use Outcome::*;

pub fn read_rps_input() -> Vec<String> {
    let file = File::open("2_data.txt").unwrap();
    let reader = BufReader::new(file);
    reader.lines().map(|a| a.unwrap()).collect()
}

pub fn rps_sim() {
    let mut score = 0;
    for line in read_rps_input() {
        let v: Vec<char> = line.chars().collect();
        let game = Game {
            my_input: convert_to_rps(v[2]),
            opp_choice: convert_to_rps(v[0]),
        };
        score += outcome(&game) + selection_score(&game.my_input)
    }
    println!("Total score: {}", score)
}

pub fn rps_sim_2() {
    let mut score = 0;
    for line in read_rps_input() {
        let v: Vec<char> = line.chars().collect();
        let opp_choice = convert_to_rps(v[0]);
        let outcome = convert_to_outcome(v[2]);
        let my_choice = given_outcome(&outcome, opp_choice);
        score += selection_score(&my_choice) + &outcome.to_score()
    }
    println!("Total score 2: {}", score)
}

pub fn given_outcome(outcome: &Outcome, opp_choice: Choice) -> Choice {
    match outcome {
        Draw => opp_choice,
        Win => {
            if opp_choice == Rock {
                Paper
            } else if opp_choice == Paper {
                Scissors
            } else {
                Rock
            }
        }
        Lose => {
            if opp_choice == Rock {
                Scissors
            } else if opp_choice == Paper {
                Rock
            } else {
                Paper
            }
        }
    }
}

pub struct Game {
    my_input: Choice,
    opp_choice: Choice,
}

#[derive(PartialEq, Eq)]
pub enum Choice {
    Rock,
    Paper,
    Scissors,
}

pub fn convert_to_rps(move_char: char) -> Choice {
    match move_char {
        'A' | 'X' => Rock,
        'B' | 'Y' => Paper,
        _ => Scissors,
    }
}

pub enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Outcome {
    pub fn to_score(&self) -> u32 {
        match self {
            Win => 6,
            Draw => 3,
            Lose => 0,
        }
    }
}

pub fn convert_to_outcome(input_char: char) -> Outcome {
    match input_char {
        'A' | 'X' => Lose,
        'B' | 'Y' => Draw,
        _ => Win,
    }
}

pub fn outcome(game: &Game) -> u32 {
    let my = &game.my_input;
    let opp = &game.opp_choice;
    if my == opp {
        return 3;
    } else if my == &Rock && opp == &Paper {
        return 0;
    } else if my == &Paper && opp == &Scissors {
        return 0;
    } else if my == &Scissors && opp == &Rock {
        return 0;
    } else {
        return 6;
    }
}

pub fn selection_score(my_choice: &Choice) -> u32 {
    match my_choice {
        Rock => 1,
        Paper => 2,
        Scissors => 3,
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    pub fn selection_score_works() {
        assert_eq!(selection_score(&Rock), 1);
        assert_eq!(selection_score(&Paper), 2);
        assert_eq!(selection_score(&Scissors), 3);
    }
    #[test]
    pub fn outcome_score_works() {
        let should_win = Game {
            my_input: Scissors,
            opp_choice: Paper,
        };
        let should_draw = Game {
            my_input: Paper,
            opp_choice: Paper,
        };
        let should_lose = Game {
            my_input: Scissors,
            opp_choice: Rock,
        };
        assert_eq!(outcome(&should_win), 6);
        assert_eq!(outcome(&should_draw), 3);
        assert_eq!(outcome(&should_lose), 0);
    }
}
