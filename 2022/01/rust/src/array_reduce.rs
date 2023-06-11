use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

pub fn run() {
    if let Ok(lines) = buffer_lines("./list") {
        let calories = lines.map(|line| line.unwrap()).fold(Vec::new(), |mut carry: Vec<i32>, line: String| {
            if line == "" {
                carry.push(0)
            } else {
                let current_calories = i32::from_str(&line).unwrap();
                match carry.last_mut() {
                    Some(calories) => *calories = *calories + current_calories,
                    None => carry.push(current_calories)
                }
            }
            return carry;
        });
        let highest_calories = calories.iter().max().unwrap();
        let elf_with_highest_calories = calories.iter().position(|value| value == highest_calories).unwrap() + 1;

        println!("Elf {} has the most calories with {} calories", elf_with_highest_calories, highest_calories);
    }
}

fn buffer_lines<TPath>(filename: TPath) -> io::Result<io::Lines<io::BufReader<File>>>
    where TPath: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
