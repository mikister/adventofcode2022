use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

enum Choice {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Win,
    Lose,
    Draw,
}

fn main() -> std::io::Result<()> {
    let file = File::open("./inputs")?;
    // let file = File::open("./inputs_small")?;

    let buf_reader = BufReader::new(file);

    let mut score: i32 = 0;

    // 1 A X Rock
    // 2 B Y Paper
    // 3 C Z Scissors

    for line_promise in buf_reader.lines() {
        let line = line_promise.unwrap();

        let enemy = match line.chars().nth(0).unwrap() {
            'A' => Choice::Rock,
            'B' => Choice::Paper,
            'C' => Choice::Scissors,
            _ => Choice::Rock,
        };

        let player = match line.chars().nth(2).unwrap() {
            'X' => Choice::Rock,
            'Y' => Choice::Paper,
            'Z' => Choice::Scissors,
            _ => Choice::Rock,
        };

        let outcome = match line.chars().nth(2).unwrap() {
            'X' => Outcome::Lose,
            'Y' => Outcome::Draw,
            'Z' => Outcome::Win,
            _ => Outcome::Win,
        };

        // score += interpret_line_as_choice(enemy, player);
        score += interpret_line_as_outcome(enemy, outcome);
        println!("Your score: {}\n", score);
    }

    println!("Your score: {}\n", score);
    Ok(())
}

fn interpret_line_as_choice(enemy: Choice, player: Choice) -> i32 {
    let score;

    match enemy {
        Choice::Rock => match player {
            Choice::Rock => score = 3 + 1,
            Choice::Paper => score = 6 + 2,
            Choice::Scissors => score = 0 + 3,
            _ => score = 0,
        },
        Choice::Paper => match player {
            Choice::Rock => score = 0 + 1,
            Choice::Paper => score = 3 + 2,
            Choice::Scissors => score = 6 + 3,
            _ => score = 0,
        },
        Choice::Scissors => match player {
            Choice::Rock => score = 6 + 1,
            Choice::Paper => score = 0 + 2,
            Choice::Scissors => score = 3 + 3,
            _ => score = 0,
        },
        _ => return 0,
    }

    return score;
}

fn interpret_line_as_outcome(enemy: Choice, outcome: Outcome) -> i32 {
    let score;

    match enemy {
        Choice::Rock => match outcome {
            Outcome::Win => score = 6 + 2,
            Outcome::Draw => score = 3 + 1,
            Outcome::Lose => score = 0 + 3,
            _ => score = 0,
        },
        Choice::Paper => match outcome {
            Outcome::Win => score = 6 + 3,
            Outcome::Draw => score = 3 + 2,
            Outcome::Lose => score = 0 + 1,
            _ => score = 0,
        },
        Choice::Scissors => match outcome {
            Outcome::Win => score = 6 + 1,
            Outcome::Draw => score = 3 + 3,
            Outcome::Lose => score = 0 + 2,
            _ => score = 0,
        },
        _ => return 0,
    }

    return score;
}
