use std::fs;
use std::io;
use std::io::BufRead;

fn main() {
    println!("Answer: {}", solve_puzzle("input.txt"));
}


#[test]
fn test_solve_puzzle() {
    assert_eq!(solve_puzzle("example.txt"), 2);
}


fn solve_puzzle(file_path: &str) -> i32 {
    let file = match fs::File::open(file_path) {
        Ok(file) => file,
        _ => panic!("File {} could not be opened.", file_path)
    };
    let reader = io::BufReader::new(file);

    let mut fully_contained_pairs = 0;

    for line in reader.lines().map(|line| line.unwrap()) {

        let mut assignments = line.split(",");
        let assignment_a = assignments.next().unwrap();
        let assignment_b = assignments.next().unwrap();

        if do_assignments_overlap(assignment_a, assignment_b) | do_assignments_overlap(assignment_b, assignment_a) {
            fully_contained_pairs += 1;
        }

    }

    return fully_contained_pairs;
}

fn do_assignments_overlap(assignment_a: &str, assignment_b: &str) -> bool {
    let range_a = to_range(assignment_a);
    let range_b = to_range(assignment_b);

    if range_a[0] <= range_b[0] && range_a[1] >= range_b[1] {
        return true;
    }
    false
}

fn to_range(assignment: &str) -> [u32; 2] {

    let mut range: [u32; 2] = [0, 0];
    for (i, range_val) in assignment.split("-").enumerate() {
        println!("{}", range_val);
        range[i] = range_val.parse::<u32>().unwrap();
    }
    range
}
