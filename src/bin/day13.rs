use std::fmt::Display;

use anyhow::Result;
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{line_ending, multispace1},
    combinator::{map, map_res},
    multi::{count, separated_list0, separated_list1},
    sequence::{delimited, separated_pair},
    IResult,
};
#[derive(Debug, PartialEq, Eq, Clone)]
enum Packet {
    Value(usize),
    Packet(Vec<Packet>),
}
impl Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Packet::Value(x) => write!(f, "{}", x),
            Packet::Packet(y) => write!(f, "[{}]", y.iter().map(|x| format!("{x}")).join(",")),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(match (self, other) {
            (Packet::Value(x), Packet::Value(y)) => return Some(x.cmp(y)),
            (left @ Packet::Value(_), Packet::Packet(right)) => right
                .get(0)
                .and_then(|y| {
                    left.partial_cmp(y).map(|ord| match ord {
                        std::cmp::Ordering::Equal => {
                            if right.len() == 1 {
                                std::cmp::Ordering::Equal
                            } else {
                                std::cmp::Ordering::Less
                            }
                        }
                        other => other,
                    })
                })
                .unwrap_or(std::cmp::Ordering::Greater),
            (Packet::Packet(left), right @ Packet::Value(_)) => left
                .get(0)
                .and_then(|x| {
                    x.partial_cmp(right).map(|ord| match ord {
                        std::cmp::Ordering::Equal => {
                            if left.len() == 1 {
                                std::cmp::Ordering::Equal
                            } else {
                                std::cmp::Ordering::Greater
                            }
                        }
                        other => other,
                    })
                })
                .unwrap_or(std::cmp::Ordering::Less),
            (Packet::Packet(left), Packet::Packet(right)) => left
                .iter()
                .zip_longest(right.iter())
                .map(|packets| match packets {
                    itertools::EitherOrBoth::Left(_) => std::cmp::Ordering::Greater,
                    itertools::EitherOrBoth::Right(_) => std::cmp::Ordering::Less,
                    itertools::EitherOrBoth::Both(x, y) => x.partial_cmp(y).unwrap(),
                })
                .find(|x| *x != std::cmp::Ordering::Equal)
                .unwrap_or(std::cmp::Ordering::Equal),
        })
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn digit1(input: &str) -> IResult<&str, usize> {
    map_res(nom::character::complete::digit1, str::parse)(input)
}

fn packet(input: &str) -> IResult<&str, Packet> {
    alt((
        map(digit1, Packet::Value),
        map(
            delimited(tag("["), separated_list0(tag(","), packet), tag("]")),
            Packet::Packet,
        ),
    ))(input)
}

fn parse_part1(input: &str) -> IResult<&str, Vec<(Packet, Packet)>> {
    separated_list1(
        count(line_ending, 2),
        separated_pair(packet, line_ending, packet),
    )(input)
}

fn parse_part2(input: &str) -> IResult<&str, Vec<Packet>> {
    separated_list1(multispace1, packet)(input)
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("inputs/day13.txt")?;
    let (remaining, packets) = parse_part1(&input).unwrap();
    assert_eq!(remaining.trim(), "");
    let part1: usize = packets
        .iter()
        .enumerate()
        .filter_map(|(i, (l, r))| if l <= r { Some(i + 1) } else { None })
        .sum();
    println!("13.1 {part1}");

    let (remaining, mut packets) = parse_part2(&input).unwrap();
    assert_eq!(remaining.trim(), "");
    let two_packet = packet("[[2]]").unwrap().1;
    let six_packet = packet("[[6]]").unwrap().1;
    packets.push(two_packet.clone());
    packets.push(six_packet.clone());
    packets.sort();

    let part2 = (packets.iter().position(|x| *x == two_packet).unwrap() + 1)
        * (packets.iter().position(|x| *x == six_packet).unwrap() + 1);

    println!("13.2: {part2}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use itertools::assert_equal;

    use super::*;

    #[test]
    fn test_part1_input() {
        let input = std::fs::read_to_string("test_inputs/day13.txt").unwrap();
        let (remaining, packets) = parse_part1(&input).unwrap();
        assert_eq!(remaining.trim(), "");
        assert_equal(
            [true, true, false, true, false, true, false, false],
            packets.into_iter().map(|(l, r)| l < r),
        );
    }

    #[test]
    fn test_part2_input() {
        let input = std::fs::read_to_string("test_inputs/day13.txt").unwrap();
        let (remaining, mut packets) = parse_part2(&input).unwrap();
        assert_eq!(remaining.trim(), "");
        packets.push(packet("[[2]]").unwrap().1);
        packets.push(packet("[[6]]").unwrap().1);
        packets.sort();

        assert_eq!(
            packets,
            parse_part2(
                "[]
            [[]]
            [[[]]]
            [1,1,3,1,1]
            [1,1,5,1,1]
            [[1],[2,3,4]]
            [1,[2,[3,[4,[5,6,0]]]],8,9]
            [1,[2,[3,[4,[5,6,7]]]],8,9]
            [[1],4]
            [[2]]
            [3]
            [[4,4],4,4]
            [[4,4],4,4,4]
            [[6]]
            [7,7,7]
            [7,7,7,7]
            [[8,7,6]]
            [9]"
            )
            .unwrap()
            .1
        );
    }
}