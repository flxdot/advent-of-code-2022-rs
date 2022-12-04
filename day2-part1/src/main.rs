use std::fs;
use std::io;
use std::io::BufRead;

fn main() -> io::Result<()> {
    let file = fs::File::open("input.txt").expect("File could not be opened");
    let reader = io::BufReader::new(file);

    let mut score: i32 = 0;

    for line in reader.lines() {
        let line = line?;
        let moves: Vec<&str> = line.split(" ").collect();

        // A/Y = Rock
        // B/X = Paper
        // C/Z = Scissors
        let (opponent, player) = (moves[0], moves[1]);

        let opponent_value = move_to_value(opponent.chars().next().unwrap(), 'A');
        let player_value = move_to_value(player.chars().next().unwrap(), 'X');

        let move_value = player_value - opponent_value;

        println!(
            "Opponent: {} ({}) | Player: {} ({}) | {}",
            opponent, opponent_value, player, player_value, move_value
        );

        score += match move_value {
            1 | -2 => 6,
            0 => 3,
            _ => 0,
        };

        score += player_value;
    }

    println!("Score: {}.", score);

    Ok(())
}

fn move_to_value(move_: char, start_char: char) -> i32 {
    //! converts a given
    move_ as i32 - start_char as i32 + 1
}
