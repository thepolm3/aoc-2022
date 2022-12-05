use std::fs;

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, line_ending},
    combinator::{map, map_res},
    multi::{count, many1, separated_list0, separated_list1},
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};

#[derive(Debug, PartialEq, Eq)]
struct Move {
    number: usize,
    from: usize,
    to: usize,
}

fn digit1(input: &str) -> IResult<&str, usize> {
    map_res(nom::character::complete::digit1, str::parse)(input)
}

fn item(input: &str) -> IResult<&str, Option<char>> {
    alt((
        map(delimited(tag("["), anychar, tag("]")), Some),
        map(count(tag(" "), 3), |_| None),
    ))(input)
}

fn stack_line(input: &str) -> IResult<&str, Vec<Option<char>>> {
    separated_list1(tag(" "), item)(input)
}

fn labels(input: &str) -> IResult<&str, Vec<usize>> {
    delimited(tag(" "), separated_list1(many1(tag(" ")), digit1), tag(" "))(input)
}

fn stacks(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    map(
        terminated(
            separated_list0(line_ending, stack_line),
            tuple((line_ending, labels, line_ending)),
        ),
        |lists| {
            let mut stacks = vec![vec![]; lists.last().map(Vec::len).unwrap_or(0)];
            for list in lists.into_iter() {
                for (i, elem) in list.into_iter().enumerate() {
                    if let Some(x) = elem {
                        stacks[i].push(x);
                    }
                }
            }
            stacks
        },
    )(input)
}

fn instruction(input: &str) -> IResult<&str, Move> {
    map(
        tuple((
            preceded(tag("move "), digit1),
            preceded(tag(" from "), digit1),
            preceded(tag(" to "), digit1),
        )),
        |(number, from, to)| Move { number, from, to },
    )(input)
}

fn instructions(input: &str) -> IResult<&str, Vec<Move>> {
    separated_list0(line_ending, instruction)(input)
}

//top of stack is at index 0, which makes the parts easier
fn parse(input: &str) -> IResult<&str, (Vec<Vec<char>>, Vec<Move>)> {
    tuple((terminated(stacks, line_ending), instructions))(input)
}

pub fn main() {
    let input = fs::read_to_string("inputs/day5.txt").unwrap();
    let (remaining, (stack, instructions)) = parse(&input).unwrap();
    assert!(remaining.chars().all(char::is_whitespace));

    println!("4.1: {}", part1(&stack, &instructions));
    println!("4.2: {}", part2(&stack, &instructions));
}

fn part1(stack: &[Vec<char>], instructions: &[Move]) -> String {
    let mut index = (1..=stack.len()).map(|x| (x, 0)).collect_vec();

    for instruction in instructions.iter().rev() {
        for (stack, depth) in &mut index {
            if *stack == instruction.from {
                *depth += instruction.number;
            }
            if *stack == instruction.to {
                if instruction.number > *depth {
                    (*stack, *depth) = (instruction.from, (instruction.number - *depth) - 1);
                } else {
                    *depth -= instruction.number;
                }
            }
        }
    }

    index.iter().map(|(i, x)| stack[i - 1][*x]).collect()
}

fn part2(stack: &[Vec<char>], instructions: &[Move]) -> String {
    let mut index = (1..=stack.len()).map(|x| (x, 0)).collect_vec();
    for instruction in instructions.iter().rev() {
        for (stack, depth) in &mut index {
            if *stack == instruction.from {
                *depth += instruction.number;
            }
            if *stack == instruction.to {
                if instruction.number > *depth {
                    (*stack, *depth) = (instruction.from, *depth);
                } else {
                    *depth -= instruction.number;
                }
            }
        }
    }

    index.iter().map(|(i, x)| stack[i - 1][*x]).collect()
}

#[cfg(test)]
mod tests {

    use itertools::Itertools;

    use super::*;

    #[test]
    fn test_parse_item() {
        let x = item("[3]").unwrap().1;
        let y = item("   ").unwrap().1;
        assert_eq!(x, Some('3'));
        assert_eq!(y, None);
    }
    #[test]
    fn test_parse_item_line() {
        let x = stack_line("[2] [3]").unwrap().1;
        assert_eq!(x, vec![Some('2'), Some('3')]);

        let y = stack_line("[2]     [3]").unwrap().1;
        assert_eq!(y, vec![Some('2'), None, Some('3')]);

        assert_eq!(
            stack_line("    [D]    ").unwrap().1,
            [None, Some('D'), None]
        )
    }
    #[test]
    fn test_parse_label_line() {
        let x = labels(" 1   2   3 ").unwrap().1;
        assert_eq!(x, vec![1, 2, 3]);

        let y = labels(" 1   2   3   4   5   6   7   8   9 ").unwrap().1;
        assert_eq!(y, vec![1, 2, 3, 4, 5, 6, 7, 8, 9,]);
    }

    #[test]
    fn test_parse_stacks() {
        let input: String = fs::read_to_string("test_inputs/day5.txt")
            .unwrap()
            .lines()
            .take(5)
            .collect_vec()
            .join("\n");
        let x = stacks(&input).unwrap().1;
        assert_eq!(vec![vec!['N', 'Z'], vec!['D', 'C', 'M'], vec!['P']], x)
    }

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("test_inputs/day5.txt").unwrap();
        let (remaining, (stack, instructions)) = parse(&input).unwrap();
        assert!(remaining.chars().all(char::is_whitespace));

        assert_eq!("CMZ", part1(&stack, &instructions));
    }

    #[test]
    fn test_part2() {
        let input = fs::read_to_string("test_inputs/day5.txt").unwrap();
        let (remaining, (stack, instructions)) = parse(&input).unwrap();
        assert!(remaining.chars().all(char::is_whitespace));

        assert_eq!("MCD", part2(&stack, &instructions));
    }
}
