use common::prelude::{glam::IVec2, *};

const INPUT: &str = include_str!("../input.txt");

fn part1(input: &str) -> i64 {
    let positions = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, value)| {
                (value == '@').then_some(IVec2::new(x as i32, y as i32))
            })
        })
        .collect::<HashSet<IVec2>>();
    dbg!(&positions);

    let count = positions
        .iter()
        .filter(|&position| {
            NEIGHBORS
                .iter()
                .filter(|&offset| positions.contains(&(position + offset)))
                .count()
                < 4
        })
        .count();

    dbg!(&count);

    count as i64
}

fn part2(input: &str) -> i64 {
    // TODO: Implement part 2
    0
}

const NEIGHBORS: [IVec2; 8] = [
    IVec2::X,
    IVec2::Y,
    IVec2::NEG_X,
    IVec2::NEG_Y,
    IVec2::ONE,
    IVec2::NEG_ONE,
    IVec2::new(1, -1),
    IVec2::new(-1, 1),
];

fn test_example(input: &str) -> i64 {
    let positions = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, value)| {
                (value == '@').then_some(IVec2::new(x as i32, y as i32))
            })
        })
        .collect::<HashSet<IVec2>>();
    dbg!(&positions);

    let count = positions
        .iter()
        .filter(|&position| {
            NEIGHBORS
                .iter()
                .filter(|&offset| positions.contains(&(position + offset)))
                .count()
                < 4
        })
        .count();

    count as i64
}

fn main() {
    println!(" Day 04 ");
    println!("Part 1: {}", part1(INPUT));
    println!("Part 2: {}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
"#;

    #[test]
    fn test_example_part1() {
        assert_eq!(test_example(EXAMPLE.trim()), 13);
    }

    #[test]
    fn test_input_part1() {
        assert_eq!(part1(INPUT), 1451);
    }

    #[test]
    fn test_input_part2() {
        assert_eq!(part2(INPUT), 0);
    }
}
