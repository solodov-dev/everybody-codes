use helpers::read_input;
use solutions::*;

fn main() {
    let solutions = [part1, part2, part3];
    for part in 0..3 {
        println!(
            "Quest 1, part {}: {}",
            part + 1,
            solutions[part](&read_input(part + 1))
        );
    }
}

mod solutions {
    use super::helpers::*;

    pub fn part1(input: &str) -> usize {
        input.trim().chars().map(matcher).sum()
    }

    pub fn part2(input: &str) -> usize {
        input
            .trim()
            .chars()
            .collect::<Vec<char>>()
            .chunks(2)
            .map(|chunk| {
                let sum = sum_chunk(chunk);
                match count_x(chunk) {
                    0 => sum + 2,
                    _ => sum,
                }
            })
            .sum()
    }

    pub fn part3(input: &str) -> usize {
        input
            .trim()
            .chars()
            .collect::<Vec<char>>()
            .chunks(3)
            .map(|chunk| {
                let sum = sum_chunk(chunk);
                match count_x(chunk) {
                    0 => sum + 6,
                    1 => sum + 2,
                    _ => sum,
                }
            })
            .sum()
    }
}

mod helpers {
    use std::fs;

    pub fn read_input(part: usize) -> String {
        fs::read_to_string(format!("../input-part-{part}")).unwrap()
    }

    pub fn matcher(ch: char) -> usize {
        match ch {
            'A' => 0,
            'B' => 1,
            'C' => 3,
            'D' => 5,
            'x' => 0,
            d => panic!("Unknown input {d}"),
        }
    }

    pub fn count_x(chunk: &[char]) -> usize {
        chunk.iter().filter(|c| **c == 'x').count()
    }

    pub fn sum_chunk(chunk: &[char]) -> usize {
        chunk.iter().map(|ch| matcher(*ch)).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("ABBAC"), 5);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("AxBCDDCAxD"), 28);
    }

    #[test]
    fn test_part3() {
        assert_eq!(part3("xBxAAABCDxCC"), 30);
    }
}
