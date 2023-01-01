use std::fs::File;
use std::io::{BufRead, BufReader};


fn main() {
    println!("{}", solve_puzzle("input.txt"));
}


fn solve_puzzle(file_path: &str) -> u32 {

    let mut group: Vec<String> = Vec::new();
    let mut group_cnt = 0;

    let mut priority_sum = 0;
    for line in read_lines(&file_path) {

        group.push(line);
        group_cnt += 1;

        if group_cnt == 3 {
            priority_sum += get_priority_of_group(&group);

            group_cnt = 0;
            group.clear();
        };
    };
    return priority_sum;
}

fn get_priority_of_group(group: &Vec<String>) -> u32 {

    for letter in b'A'..=b'z' {
        let c = letter as char;
        if group.iter().all(|group| group.contains(c)) {
            print!("Found character {}\n", c);
            return calc_priority(c);
        }
    }

    return 0;
}

fn calc_priority(c: char) -> u32 {
    c as u32 - match c.is_lowercase() {
        true => 96,
        false => 38
    }
}

#[test]
fn test_on_example() {
    assert_eq!(solve_puzzle("example.txt"), 70);
}

/// Reads the lines of the file at the given path and returns an iterator over the lines.
///
/// # Examples
///
/// ```
/// let file_path = "path/to/file.txt";
/// let lines = read_lines(file_path);
///
/// for line in lines {
///     println!("{}", line);
/// }
/// ```
///
/// # Errors
///
/// This function will return an error if the file at the given path cannot be opened or read.
///
/// # Panics
///
/// This function will panic if any of the lines returned by the iterator are `Err` values.
fn read_lines(file_path: &str) -> impl Iterator<Item = String> {
    let file = match File::open(file_path) {
        Ok(reader) => reader,
        Err(..) => panic!("File not found!")
    };
    let reader = BufReader::new(file);

    reader.lines().map(|line| line.unwrap())
}