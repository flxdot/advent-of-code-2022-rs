use rstest::rstest;
use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut content = Vec::new();
    
    file.read_to_end(&mut content)?;

    //println!("Part 1: {}", find_marker_pos(content, 4));
    println!("Part 2: {}", find_marker_pos(content, 14));

    return Ok(());
}

fn find_marker_pos(signal: Vec<u8>, distinc_len: usize) -> usize {
    for (idx, set) in signal.windows(distinc_len).enumerate() {
        let mut seen: u32 = 0;
        for b in set {
            seen |= 1 << (b % 32);
            if seen.count_ones() == distinc_len as u32 {
                return idx + distinc_len;
            };
        }
    }
    return 0;
}

#[rstest]
#[case("bvwbjplbgvbhsrlpgdmjqwftvncz", 5)]
#[case("nppdvjthqldpwncqszvftbrmjlhg", 6)]
#[case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10)]
#[case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11)]
fn test_part1(#[case] input: &str, #[case] expected: usize) {
    assert_eq!(expected, find_marker_pos(input.as_bytes().to_vec(), 4));
}

#[rstest]
#[case("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19)]
#[case("bvwbjplbgvbhsrlpgdmjqwftvncz", 23)]
#[case("nppdvjthqldpwncqszvftbrmjlhg", 23)]
#[case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29)]
#[case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26)]
fn test_part2(#[case] input: &str, #[case] expected: usize) {
    assert_eq!(expected, find_marker_pos(input.as_bytes().to_vec(), 14));
}
