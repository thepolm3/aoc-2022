use std::fmt::Display;

use anyhow::Result;
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::multispace1,
    combinator::{map, map_res},
    multi::{separated_list0, separated_list1},
    sequence::delimited,
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
        Some(self.cmp(other))
    }
}
impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Packet::Value(l), Packet::Value(r)) => l.cmp(r),
            (Packet::Packet(l), Packet::Packet(r)) => l.cmp(r),
            (Packet::Value(l), Packet::Packet(r)) => {
                [Packet::Value(*l)].as_slice().cmp(r.as_slice())
            }
            (Packet::Packet(l), Packet::Value(r)) => {
                l.as_slice().cmp([Packet::Value(*r)].as_slice())
            }
        }
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

fn parse(input: &str) -> IResult<&str, Vec<Packet>> {
    separated_list1(multispace1, packet)(input)
}

fn part1(packets: &Vec<Packet>) -> usize {
    packets
        .iter()
        .tuples()
        .enumerate()
        .filter_map(|(i, (l, r))| if l <= r { Some(i + 1) } else { None })
        .sum()
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("inputs/day13.txt")?;

    let (remaining, mut packets) = parse(&input).unwrap();
    assert_eq!(remaining.trim(), "");

    println!("13.1 {}", part1(&packets));

    let two_packet = packet("[[2]]").unwrap().1;
    let six_packet = packet("[[6]]").unwrap().1;
    packets.push(two_packet.clone());
    packets.push(six_packet.clone());
    packets.sort();

    let part2 = (packets.binary_search(&two_packet).unwrap() + 1)
        * (packets.binary_search(&six_packet).unwrap() + 1);

    println!("13.2: {part2}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use itertools::assert_equal;

    use super::*;

    #[test]
    fn test_part1() {
        let input = std::fs::read_to_string("test_inputs/day13.txt").unwrap();
        let (remaining, packets) = parse(&input).unwrap();
        assert_eq!(remaining.trim(), "");
        assert_equal(
            [true, true, false, true, false, true, false, false],
            packets.into_iter().tuples().map(|(l, r)| l < r),
        );
    }

    #[test]
    fn test_part2() {
        let input = std::fs::read_to_string("test_inputs/day13.txt").unwrap();
        let (remaining, mut packets) = parse(&input).unwrap();
        assert_eq!(remaining.trim(), "");
        packets.push(packet("[[2]]").unwrap().1);
        packets.push(packet("[[6]]").unwrap().1);
        packets.sort();

        assert_eq!(
            packets,
            parse(
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
