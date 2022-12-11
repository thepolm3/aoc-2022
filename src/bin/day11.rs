use std::collections::VecDeque;

use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::multispace1,
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair, tuple},
    IResult,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Variable {
    Old,
    Const(usize),
}
impl Variable {
    fn get(&self, input: usize) -> usize {
        match self {
            Variable::Old => input,
            Variable::Const(x) => *x,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Op {
    Mul,
    Add,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct MonkeyEquation {
    op: Op,
    input: Variable,
}
impl MonkeyEquation {
    fn apply(&self, old: usize) -> usize {
        let snd_term = self.input.get(old);
        match self.op {
            Op::Mul => old * snd_term,
            Op::Add => old + snd_term,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct MonkeyTest {
    divisible_by: usize,
    t: usize,
    f: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Monkey {
    id: usize,
    items: VecDeque<usize>,
    operation: MonkeyEquation,
    test: MonkeyTest,
}

fn digit1(input: &str) -> IResult<&str, usize> {
    map_res(nom::character::complete::digit1, str::parse)(input)
}

fn monkey_header(input: &str) -> IResult<&str, usize> {
    delimited(tag("Monkey "), digit1, tag(":"))(input)
}

fn starting_items(input: &str) -> IResult<&str, Vec<usize>> {
    preceded(tag("Starting items: "), separated_list1(tag(", "), digit1))(input)
}

fn variable(input: &str) -> IResult<&str, Variable> {
    alt((
        map(tag("old"), |_| Variable::Old),
        map(digit1, Variable::Const),
    ))(input)
}
fn op(input: &str) -> IResult<&str, Op> {
    alt((map(tag("+"), |_| Op::Add), map(tag("*"), |_| Op::Mul)))(input)
}

fn operation(input: &str) -> IResult<&str, MonkeyEquation> {
    map(
        preceded(
            tag("Operation: new = old "),
            separated_pair(op, tag(" "), variable),
        ),
        |(op, input)| MonkeyEquation { op, input },
    )(input)
}

fn test_header(input: &str) -> IResult<&str, usize> {
    preceded(tag("Test: divisible by "), digit1)(input)
}
fn test_line_1(input: &str) -> IResult<&str, usize> {
    preceded(tag("If true: throw to monkey "), digit1)(input)
}
fn test_line_2(input: &str) -> IResult<&str, usize> {
    preceded(tag("If false: throw to monkey "), digit1)(input)
}

fn test(input: &str) -> IResult<&str, MonkeyTest> {
    map(
        tuple((
            test_header,
            multispace1,
            test_line_1,
            multispace1,
            test_line_2,
        )),
        |(divisible_by, _, t, _, f)| MonkeyTest { divisible_by, t, f },
    )(input)
}

fn monkey(input: &str) -> IResult<&str, Monkey> {
    map(
        tuple((
            monkey_header,
            multispace1,
            starting_items,
            multispace1,
            operation,
            multispace1,
            test,
        )),
        |(id, _, items, _, operation, _, test)| Monkey {
            id,
            items: VecDeque::from(items),
            operation,
            test,
        },
    )(input)
}

fn monkeys(input: &str) -> IResult<&str, Vec<Monkey>> {
    separated_list1(multispace1, monkey)(input)
}
fn main() -> Result<()> {
    let input = std::fs::read_to_string("inputs/day11.txt").unwrap();
    let (remaining, monkeys) = monkeys(&input).unwrap();
    assert_eq!(remaining.trim(), "");
    let mut part1_monkeys = monkeys.clone();

    let mut inspections = vec![0; monkeys.len()];
    let mut monkey;
    for _round in 0..20 {
        for i in 0..part1_monkeys.len() {
            monkey = part1_monkeys[i].clone();
            inspections[i] += monkey.items.len();

            while let Some(item) = monkey.items.pop_front() {
                let worry_level = monkey.operation.apply(item) / 3;
                if worry_level % monkey.test.divisible_by == 0 {
                    part1_monkeys[monkey.test.t].items.push_back(worry_level);
                } else {
                    part1_monkeys[monkey.test.f].items.push_back(worry_level);
                }
            }
            part1_monkeys[i] = monkey;
        }
    }
    inspections.sort();
    println!(
        "11.1 {:?}",
        inspections.iter().rev().take(2).product::<usize>()
    );

    let mut part2_monkeys = monkeys.clone();
    let factor = monkeys
        .iter()
        .map(|monkey| monkey.test.divisible_by)
        .product::<usize>();

    let mut inspections = vec![0; part2_monkeys.len()];
    let mut monkey;
    for _round in 0..10000 {
        for i in 0..part2_monkeys.len() {
            monkey = part2_monkeys[i].clone();
            inspections[i] += monkey.items.len();

            while let Some(item) = monkey.items.pop_front() {
                let worry_level = monkey.operation.apply(item) % factor;
                if worry_level % monkey.test.divisible_by == 0 {
                    part2_monkeys[monkey.test.t].items.push_back(worry_level);
                } else {
                    part2_monkeys[monkey.test.f].items.push_back(worry_level);
                }
            }
            part2_monkeys[i] = monkey;
        }
    }
    inspections.sort();
    println!(
        "11.2 {:?}",
        inspections.iter().rev().take(2).product::<usize>()
    );
    Ok(())
}
