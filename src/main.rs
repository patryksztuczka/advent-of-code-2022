use std::fs;

fn main() {
    let file_content = match fs::read_to_string("input.txt") {
        Ok(f) => f,
        Err(err) => panic!("Problem reading the file: {}", err)
    };

    let mut v: Vec<u32> = Vec::new();

    let mut elf_calories: u32 = 0;

    for line in file_content.lines() {
        if line.len() == 0 {
            v.push(elf_calories);
            elf_calories = 0;
            continue;
        }

        let line_value: u32 = line.trim().parse().unwrap();

        elf_calories += line_value

    }

    v.sort();

    println!("Top 3 sum: {}", v.into_iter().rev().take(3).sum::<u32>());

}
