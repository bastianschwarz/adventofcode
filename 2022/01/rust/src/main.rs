use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

fn main() {
    struct ElfWithMostCalories {
        number: i8,
        calories: i32,
    }

    let mut elf_with_most_calories: ElfWithMostCalories = ElfWithMostCalories {
        number: 0,
        calories: 0,
    };

    let mut current_calories = 0;
    let mut current_elf: i8 = 1;

    if let Ok(lines) = buffer_lines("./list") {
        for line in lines {
            if let Ok(calories) = line {
                if calories.is_empty() {
                    if current_calories > elf_with_most_calories.calories {
                        elf_with_most_calories.calories = current_calories;
                        elf_with_most_calories.number = current_elf;
                    }

                    current_calories = 0;
                    current_elf = current_elf + 1;
                } else {
                    current_calories = current_calories + i32::from_str(&calories).unwrap();
                }
            }
        }
    }

    println!("Elf {} has the most calories with {} calories", elf_with_most_calories.number, elf_with_most_calories.calories);
}

fn buffer_lines<TPath>(filename: TPath) -> io::Result<io::Lines<io::BufReader<File>>>
    where TPath: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
