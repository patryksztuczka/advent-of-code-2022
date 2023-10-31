use std::fs;

fn main() {
    let file_content = match fs::read_to_string("input.txt") {
        Ok(f) => f,
        Err(err) => panic!("Problem reading the file: {}", err)
    };

    let mut max_calories: u32 = 0;

    let mut elf_calories: u32 = 0;

    for line in file_content.lines() {
        if line.len() == 0 {
            if max_calories < elf_calories {
                max_calories = elf_calories;
            }
            elf_calories = 0;
            continue;
        }

        let line_value: u32 = line.trim().parse().unwrap();

        elf_calories += line_value
    }

    println!("Max calories: {}", max_calories)

}