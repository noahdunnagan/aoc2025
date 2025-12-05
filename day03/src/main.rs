use common::prelude::*;

const INPUT: &str = include_str!("../input.txt");

fn part1(input: &str) -> i64 {
    let mut joltage = 0;

    for line in input.lines() {
        let digits: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();

        let result: u32 = digits
            .iter()
            .enumerate()
            .tuple_combinations()
            .map(|((_, &a), (_, &b))| a * 10 + b)
            .max()
            .unwrap();

        joltage += result
    }

    i64::from(joltage)
}

fn part2(input: &str) -> i64 {
    // TODO: Implement part 2
    0
}

fn example_test(input: &str) -> i64 {
    let mut joltage = 0;
    dbg!(input.lines().count());
    for line in input.lines() {
        let digits: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();

        let result: u32 = digits
            .iter()
            .enumerate()
            .tuple_combinations()
            .map(|((_, &a), (_, &b))| a * 10 + b) // a comes before b
            .max()
            .unwrap();

        joltage += result
    }
    i64::from(joltage)
}

fn main() {
    println!(" Day 03 ");
    println!("Part 1: {}", part1(INPUT));
    println!("Part 2: {}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"
987654321111111
811111111111119
234234234234278
818181911112111
"#;

    #[test]
    fn test_example_input() {
        assert_eq!(example_test(EXAMPLE.trim()), 357);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT.trim()), 17432);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE.trim()), 0);
    }
}
