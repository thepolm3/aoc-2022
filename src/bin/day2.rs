use std::fs;

use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, one_of},
    multi::separated_list0,
    sequence::tuple,
    IResult,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Throw {
    Rock = 0,
    Paper = 1,
    Scissors = 2,
}
impl Throw {
    fn points(&self) -> usize {
        *self as usize + 1
    }

    //the outcome of self playing other, from the perspective of self
    fn play(&self, other: &Self) -> Outcome {
        match (3 + *self as usize - *other as usize) % 3 {
            0 => Outcome::Draw,
            1 => Outcome::Win,
            2 => Outcome::Lose,
            _ => unreachable!(),
        }
    }

    //what to play to force a specific outcome
    fn force(&self, outcome: &Outcome) -> Throw {
        match (*self as usize + *outcome as usize) % 3 {
            1 => Throw::Rock,
            2 => Throw::Paper,
            0 => Throw::Scissors,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Outcome {
    Lose = 0,
    Draw = 1,
    Win = 2,
}
impl Outcome {
    fn points(&self) -> usize {
        (*self as usize) * 3
    }
}

fn parse1(input: &str) -> IResult<&str, Vec<(Throw, Throw)>> {
    separated_list0(
        line_ending,
        nom::combinator::map(
            tuple((one_of("ABC"), tag(" "), one_of("XYZ"))),
            |(abc, _, xyz)| {
                (
                    match abc {
                        'A' => Throw::Rock,
                        'B' => Throw::Paper,
                        'C' => Throw::Scissors,
                        _ => unreachable!(),
                    },
                    match xyz {
                        'X' => Throw::Rock,
                        'Y' => Throw::Paper,
                        'Z' => Throw::Scissors,
                        _ => unreachable!(),
                    },
                )
            },
        ),
    )(input)
}

fn parse2(input: &str) -> IResult<&str, Vec<(Throw, Outcome)>> {
    separated_list0(
        line_ending,
        nom::combinator::map(
            tuple((one_of("ABC"), tag(" "), one_of("XYZ"))),
            |(abc, _, xyz)| {
                (
                    match abc {
                        'A' => Throw::Rock,
                        'B' => Throw::Paper,
                        'C' => Throw::Scissors,
                        _ => unreachable!(),
                    },
                    match xyz {
                        'X' => Outcome::Lose,
                        'Y' => Outcome::Draw,
                        'Z' => Outcome::Win,
                        _ => unreachable!(),
                    },
                )
            },
        ),
    )(input)
}

pub fn main() {
    let input = fs::read_to_string("inputs/day2.txt").unwrap();
    let data = parse1(&input).unwrap().1;
    println!("2.1: {}", part1(&data));

    let data = parse2(&input).unwrap().1;
    println!("2.2: {}", part2(&data));
}

fn part1(throws: &[(Throw, Throw)]) -> usize {
    throws
        .iter()
        .map(|(first, second)| second.play(first).points() + second.points())
        .sum()
}

fn part2(throws: &[(Throw, Outcome)]) -> usize {
    throws
        .iter()
        .map(|(first, outcome)| first.force(outcome).points() + outcome.points())
        .sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part1() {
        let test_input = fs::read_to_string("test_inputs/day2.txt").unwrap();
        let (remaining, data) = parse1(&test_input).unwrap();
        eprintln!("{:?}", data);
        assert_eq!(remaining, "");
        assert_eq!(part1(&data), 15);
    }

    #[test]
    fn test_part2() {
        let test_input = fs::read_to_string("test_inputs/day2.txt").unwrap();
        let (remaining, data) = parse2(&test_input).unwrap();
        eprintln!("{:?}", data);
        assert_eq!(remaining, "");
        assert_eq!(part2(&data), 12);
    }

    #[test]
    fn test_play() {
        assert_eq!(Throw::play(&Throw::Rock, &Throw::Rock), Outcome::Draw);
        assert_eq!(Throw::play(&Throw::Paper, &Throw::Rock), Outcome::Win);
        assert_eq!(Throw::play(&Throw::Scissors, &Throw::Rock), Outcome::Lose);
        assert_eq!(Throw::play(&Throw::Rock, &Throw::Paper), Outcome::Lose);
        assert_eq!(Throw::play(&Throw::Paper, &Throw::Paper), Outcome::Draw);
        assert_eq!(Throw::play(&Throw::Scissors, &Throw::Paper), Outcome::Win);
        assert_eq!(Throw::play(&Throw::Rock, &Throw::Scissors), Outcome::Win);
        assert_eq!(Throw::play(&Throw::Paper, &Throw::Scissors), Outcome::Lose);
        assert_eq!(
            Throw::play(&Throw::Scissors, &Throw::Scissors),
            Outcome::Draw
        );
    }

    #[test]
    fn test_force() {
        assert_eq!(Throw::Rock.force(&Outcome::Draw), Throw::Rock);
        assert_eq!(Throw::Rock.force(&Outcome::Win), Throw::Paper);
        assert_eq!(Throw::Rock.force(&Outcome::Lose), Throw::Scissors);
        assert_eq!(Throw::Paper.force(&Outcome::Lose), Throw::Rock);
        assert_eq!(Throw::Paper.force(&Outcome::Draw), Throw::Paper);
        assert_eq!(Throw::Paper.force(&Outcome::Win), Throw::Scissors);
        assert_eq!(Throw::Scissors.force(&Outcome::Win), Throw::Rock);
        assert_eq!(Throw::Scissors.force(&Outcome::Lose), Throw::Paper);
        assert_eq!(Throw::Scissors.force(&Outcome::Draw), Throw::Scissors);
    }
}
