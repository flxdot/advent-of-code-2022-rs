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

        let (assignment_a, assignment_b) = line.split_once(",").unwrap();
        let range_a = to_range(assignment_a);
        let range_b = to_range(assignment_b);

        if do_assignments_overlap(range_a, range_b) | do_assignments_overlap(range_b, range_a) {
            fully_contained_pairs += 1;
        }
    }
    Ok(fully_contained_pairs)
}

fn to_range(assignment: &str) -> [u32; 2] {
    let range_str = assignment.split_once("-").unwrap();
    [range_str.0.parse::<u32>().unwrap(), range_str.1.parse::<u32>().unwrap()]
}

fn do_assignments_overlap(range_a: [u32; 2], range_b: [u32; 2]) -> bool {
    range_a[0] <= range_b[0] && range_a[1] >= range_b[1]
}
