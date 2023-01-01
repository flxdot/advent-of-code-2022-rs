use std::fs;
use std::io;
use std::io::BufRead;

fn main() {
    println!("Answer: {}", solve_puzzle("input.txt").unwrap());
}


#[test]
fn test_solve_puzzle() {
    assert_eq!(solve_puzzle("example.txt").unwrap(), 2);
}


fn solve_puzzle(file_path: &str) -> Result<i32, std::io::Error> {
    let file = fs::File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut fully_contained_pairs = 0;

    for line in reader.lines() {
        let line = line?;

        let assignments = line.split_once(",").unwrap();
        let range_a = to_range(assignments.0);
        let range_b = to_range(assignments.1);

        if is_range_contained_in(range_a, range_b) || is_range_contained_in(range_b, range_a) {
            fully_contained_pairs += 1;
        }
    }

    Ok(fully_contained_pairs)
}

fn to_range(assignment: &str) -> [u32; 2] {
    let range_str = assignment.split_once("-").unwrap();
    [range_str.0, range_str.1].map(|v| v.parse::<u32>().unwrap())
}

fn is_range_contained_in(reference: [u32; 2], other: [u32; 2]) -> bool {
    reference[0] <= other[0] && reference[1] >= other[1]
}
