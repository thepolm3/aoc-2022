use std::{collections::HashSet, fs};

use itertools::Itertools;

pub fn main() {
    let input = fs::read_to_string("inputs/day3.txt").unwrap();
    let data = parse1(&input);
    println!("3.1: {}", part1(&data));

    let data = parse2(&input);
    println!("3.2: {}", part2(&data).unwrap());
}

fn parse1(input: &str) -> Vec<(&str, &str)> {
    input
        .lines()
        .map(|line| line.split_at(line.len() / 2))
        .collect()
}
fn parse2(input: &str) -> Vec<Vec<&str>> {
    input
        .lines()
        .into_iter()
        .chunks(3)
        .into_iter()
        .map(Itertools::collect_vec)
        .collect_vec()
}

fn get_value(c: char) -> usize {
    match (c.to_ascii_uppercase(), c.is_uppercase()) {
        (x, true) => x as usize - 64 + 26,
        (x, false) => x as usize - 64,
    }
}

fn part1(input: &[(&str, &str)]) -> usize {
    input
        .iter()
        .map(|(a, b)| match a.chars().find(|a_i| b.contains(*a_i)) {
            Some(x) => get_value(x),
            None => 0,
        })
        .sum()
}

fn part2(input: &[Vec<&str>]) -> Option<usize> {
    input
        .iter()
        .map(|backpacks| -> Option<usize> {
            Some(get_value(
                backpacks
                    .iter()
                    .map(|backpack| HashSet::<char>::from_iter(backpack.chars()))
                    .reduce(|acc, val| acc.intersection(&val).copied().collect())?
                    .into_iter()
                    .next()?,
            ))
        })
        .filter(Option::is_some)
        .sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part1() {
        let test_input = fs::read_to_string("test_inputs/day3.txt").unwrap();
        let data = parse1(&test_input);
        eprintln!("{:?}", data);
        assert_eq!(part1(&data), 157);
    }

    #[test]
    fn test_part2() {
        let test_input = fs::read_to_string("test_inputs/day3.txt").unwrap();
        let data = parse2(&test_input);
        eprintln!("{:?}", data);
        assert_eq!(part2(&data), Some(70));
    }
}
