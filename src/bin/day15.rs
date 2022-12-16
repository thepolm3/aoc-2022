use anyhow::{Context, Result};
use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending},
    combinator::{map, map_res, opt, recognize},
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct Coord(isize, isize);

fn parse_isize(input: &str) -> IResult<&str, isize> {
    map_res(recognize(preceded(opt(tag("-")), digit1)), str::parse)(input)
}

fn sensor_beacon(input: &str) -> IResult<&str, (Coord, Coord)> {
    map(
        tuple((
            tag("Sensor at x="),
            parse_isize,
            tag(", y="),
            parse_isize,
            tag(": closest beacon is at x="),
            parse_isize,
            tag(", y="),
            parse_isize,
        )),
        |(_, x1, _, y1, _, x2, _, y2)| (Coord(x1, y1), Coord(x2, y2)),
    )(input)
}

fn d(s: &Coord, b: &Coord) -> usize {
    s.0.abs_diff(b.0) + s.1.abs_diff(b.1)
}

fn parse(input: &str) -> IResult<&str, Vec<(Coord, Coord)>> {
    separated_list1(line_ending, sensor_beacon)(input)
}

fn sorted_to_disjoint_intervals(intervals: &Vec<(isize, isize)>) -> Vec<(isize, isize)> {
    let mut result = Vec::new();
    let mut i = 0;
    while let Some((start, end)) = intervals.get(i) {
        let mut merged_end = end;
        while let Some((other_start, other_end)) = intervals.get(i) {
            if other_start > merged_end {
                break;
            } else {
                merged_end = merged_end.max(other_end)
            }
            i += 1;
        }
        result.push((*start, *merged_end));
    }
    result
}
fn disjoint_intersections_with_row(
    sensor_beacons: &Vec<(Coord, Coord)>,
    y: isize,
) -> Vec<(isize, isize)> {
    let mut intersections = Vec::new();

    for (sensor, beacon) in sensor_beacons {
        let range = d(&sensor, &beacon);
        if sensor.1 + range as isize > y && sensor.1 - (range as isize) < y {
            let reach = (range - sensor.1.abs_diff(y)) as isize;
            intersections.push(((sensor.0 - reach), (sensor.0 + reach)));
        }
    }
    intersections.sort();
    sorted_to_disjoint_intervals(&intersections)
}
fn count_impossible_beacons_in_row(sensor_beacons: &Vec<(Coord, Coord)>, y: isize) -> usize {
    let intersections = disjoint_intersections_with_row(&sensor_beacons, y);

    intersections
        .iter()
        .map(|(start, end)| (end - start + 1) as usize)
        .sum::<usize>()
        - sensor_beacons //count beacons which are intersecting
            .iter()
            .filter_map(|(_, b)| (b.1 == y).then_some(b.0))
            .unique()
            .into_iter()
            .filter(|&x| {
                intersections
                    .iter()
                    .any(|(start, end)| *start <= x && x <= *end)
            })
            .count()
}
fn part2_brute_force(sensor_beacons: &Vec<(Coord, Coord)>, min: isize, max: isize) -> isize {
    //todo: get disjoit excluded rows by iterating through the sensors

    for y in min..=max {
        let intersections = disjoint_intersections_with_row(&sensor_beacons, y);
        if intersections.len() > 1 {
            return (intersections[0].1 + 1) * max + y;
        }
    }
    0
}
fn main() -> Result<()> {
    let input = std::fs::read_to_string("inputs/day15.txt")?;

    let (remaining, sensor_beacons) = parse(&input).unwrap();
    assert_eq!(remaining.trim(), "");

    println!(
        "{:?}",
        disjoint_intersections_with_row(&sensor_beacons, 3349056)
    );
    let part1 = count_impossible_beacons_in_row(&sensor_beacons, 2_000_000);
    println!("15.1: {part1}");
    println!("15.2: {}", part2_brute_force(&sensor_beacons, 0, 4_000_000));

    println!();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = std::fs::read_to_string("test_inputs/day15.txt").unwrap();

        let (remaining, sensor_beacons) = parse(&input).unwrap();
        assert_eq!(remaining.trim(), "");

        assert_eq!(count_impossible_beacons_in_row(&sensor_beacons, 10), 26);
    }
}
