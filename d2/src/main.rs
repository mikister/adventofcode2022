use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let file = File::open("./inputs")?;
    // let file = File::open("./inputs_small")?;

    let buf_reader = BufReader::new(file);

    let mut score: i64 = 0;

    // 1 A X Rock
    // 2 B Y Paper
    // 3 C Z Scissors

    for line_promise in buf_reader.lines() {
        let line = line_promise.unwrap();

        match line.chars().nth(0).unwrap() {
            'A' => {
                match line.chars().nth(2).unwrap() {
                    'X' => score += 3 + 1,
                    'Y' => score += 6 + 2,
                    'Z' => score += 0 + 3,
                    _ => println!("Bad input"),
                }
                // println!("Your score: {}\n", score);
            }

            'B' => {
                match line.chars().nth(2).unwrap() {
                    'X' => score += 0 + 1,
                    'Y' => score += 3 + 2,
                    'Z' => score += 6 + 3,
                    _ => println!("Bad input"),
                }
                // println!("Your score: {}\n", score);
            }

            'C' => {
                match line.chars().nth(2).unwrap() {
                    'X' => score += 6 + 1,
                    'Y' => score += 0 + 2,
                    'Z' => score += 3 + 3,
                    _ => println!("Bad input"),
                }
                // println!("Your score: {}\n", score);
            }

            _ => println!("Bad input"),
        }
    }

    println!("Your score: {}\n", score);
    Ok(())
}
