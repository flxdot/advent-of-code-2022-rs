use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};


fn main() -> io::Result<()> {

    let score = calculate_score_for("input.txt");

    println!("Score: {}", score);

    Ok(())
}

enum Move {
    Rock,
    Paper,
    Scissors
}

enum DesiredResult {
    Lose,
    Draw,
    Win
}

fn calculate_score_for(file_path: &str) -> i32 {
    let mut score = 0;

    for line in read_lines(file_path) {
        let moves: Vec<&str> = line.split(" ").collect();

        // A = Rock
        // B = Paper
        // C = Scissors
        let opponent = match moves[0] {
            "A" => Move::Rock,
            "B" => Move::Paper,
            _ => Move::Scissors
        };
        // X = Lose
        // Y = Draw
        // Z = Win
        let required_result = match moves[1] {
            "X" => DesiredResult::Lose,
            "Y" => DesiredResult::Draw,
            _ => DesiredResult::Win
        };

        score += match required_result {
            DesiredResult::Win => 6,
            DesiredResult::Draw => 3,
            _ => 0
        };

        let player_move = match (opponent, required_result) {
            (Move::Rock, DesiredResult::Win) => Move::Paper,
            (Move::Rock, DesiredResult::Draw) => Move::Rock,
            (Move::Rock, DesiredResult::Lose) => Move::Scissors,
            (Move::Paper, DesiredResult::Win) => Move::Scissors,
            (Move::Paper, DesiredResult::Draw) => Move::Paper,
            (Move::Paper, DesiredResult::Lose) => Move::Rock,
            (Move::Scissors, DesiredResult::Win) => Move::Rock,
            (Move::Scissors, DesiredResult::Draw) => Move::Scissors,
            _ => Move::Paper,
        };

        score += match player_move {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }

    }

    score
}

#[test]
fn test_calculate_score_for() {
    assert_eq!(calculate_score_for("example.txt"), 12);
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
