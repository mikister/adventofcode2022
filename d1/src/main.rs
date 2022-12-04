use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let file = File::open("./inputs")?;
    let buf_reader = BufReader::new(file);

    let mut calories_highest = [0, 0, 0];
    let mut calories_current = 0;

    for line_promise in buf_reader.lines() {
        let line = line_promise.unwrap();

        if line.len() == 0 {
            // print!("----------------------\n");
            // print!("{}\n", calories_current);
            for i in 0..(calories_highest.len()) {
                if calories_current > calories_highest[i] {
                    (calories_current, calories_highest[i]) =
                        (calories_highest[i], calories_current)
                }
            }
            calories_current = 0;
        } else {
            calories_current += line.parse::<i32>().unwrap();
        }
    }

    print!("----------------------\n");
    for i in 0..(calories_highest.len()) {
        print!("Highest ({}): {}\n", i + 1, calories_highest[i]);
    }
    print!("----------------------\n");

    Ok(())
}
