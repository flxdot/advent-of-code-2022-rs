use std::fs;
use std::io;
use std::io::BufRead;

fn main() -> io::Result<()> {
    let file = fs::File::open("input.txt")?;
    let reader = io::BufReader::new(file);

    let mut current = 0;
    let mut calories_per_elf = Vec::<u32>::new();

    for line in reader.lines() {
        let line = line?;

        // If line is empty, figure out if we have a new max
        if line.is_empty() {
            calories_per_elf.push(current);
            current = 0;
            continue;
        }

        match line.parse::<u32>() {
            Ok(n) => {
                current += n;
            },
            Err(e) => {
                println!("Error: {}", e);
                continue;
            }
        };
    }

    calories_per_elf.sort();
    let top_three_sum: u32 = calories_per_elf[calories_per_elf.len()-3..].iter().sum();

    print!("Top 3 elfes carry {} calories", top_three_sum);

    // Return an 'Ok' value containing '()'.
    // This indicates that the 'main' function has completed successfully.
    Ok(())
}
