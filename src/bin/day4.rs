use std::fs;

use nom::{
    bytes::complete::tag, character::complete::line_ending, combinator::map_res,
    multi::separated_list0, sequence::separated_pair, IResult,
};

fn digit1(input: &str) -> IResult<&str, usize> {
    map_res(nom::character::complete::digit1, str::parse)(input)
}

fn elf_range(input: &str) -> IResult<&str, (usize, usize)> {
    separated_pair(digit1, tag("-"), digit1)(input)
}
fn parse_line(input: &str) -> IResult<&str, ((usize, usize), (usize, usize))> {
    separated_pair(elf_range, tag(","), elf_range)(input)
}
fn parse(input: &str) -> IResult<&str, Vec<((usize, usize), (usize, usize))>> {
    separated_list0(line_ending, parse_line)(input)
}

pub fn main() {
    let input = fs::read_to_string("inputs/day4.txt").unwrap();
    let (remaining, data) = parse(&input).unwrap();
    assert!(remaining.is_empty());
    println!("4.1: {:?}", part1(&data));
    println!("4.2: {:?}", part2(&data));
}

fn part1(input: &[((usize, usize), (usize, usize))]) -> usize {
    input
        .iter()
        .filter(|((a, b), (c, d))| (a <= c && b >= d) || (c <= a && d >= b))
        .count()
}

fn part2(input: &[((usize, usize), (usize, usize))]) -> usize {
    input
        .iter()
        .filter(|((a, b), (c, d))| c <= b && a <= d)
        .count()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part1() {
        let test_input = fs::read_to_string("test_inputs/day4.txt").unwrap();
        let (remaining, data) = parse(&test_input).unwrap();
        assert_eq!(remaining, "");
        eprintln!("{:?}", data);
        assert_eq!(part1(&data), 2);
    }

    #[test]
    fn test_part2() {
        let test_input = fs::read_to_string("test_inputs/day4.txt").unwrap();
        let (remaining, data) = parse(&test_input).unwrap();
        assert_eq!(remaining, "");
        eprintln!("{:?}", data);
        assert_eq!(part2(&data), 4);
    }
}
