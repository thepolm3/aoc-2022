use std::fs;

use itertools::Itertools;
use nom::{
    character::{complete::digit0, complete::line_ending},
    combinator::map_res,
    multi::{count, separated_list0},
    IResult,
};

fn parse(input: &str) -> IResult<&str, Vec<Vec<usize>>> {
    separated_list0(
        count(line_ending, 2),
        separated_list0(
            line_ending,
            map_res(digit0, |out: &str| out.parse::<usize>()),
        ),
    )(input)
}

fn part1(lines: &[Vec<usize>]) -> Option<usize> {
    lines.iter().map(|x| x.iter().sum::<usize>()).max()
}

fn part2(lines: &[Vec<usize>]) -> usize {
    lines
        .iter()
        .map(|x| x.iter().sum::<usize>())
        .sorted()
        .into_iter()
        .rev()
        .take(3)
        .sum::<usize>()
}

pub fn main() {
    let input = fs::read_to_string("inputs/day1.txt").unwrap();
    let data = parse(&input).unwrap().1;
    println!("1.1: {}", part1(&data).unwrap());
    println!("1.2: {}", part2(&data));
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_input() {
        let test_input = std::fs::read_to_string("test_inputs/day1.txt").unwrap();
        let (remaining, data) = parse(&test_input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(part1(&data), Some(24000));

        assert_eq!(part2(&data), 45000);
    }
}
