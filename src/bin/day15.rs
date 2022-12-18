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

fn sorted_to_disjoint_intervals(intervals: &[(isize, isize)]) -> Vec<(isize, isize)> {
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
        let range = d(sensor, beacon);
        if sensor.1 + range as isize > y && sensor.1 - (range as isize) < y {
            let reach = (range - sensor.1.abs_diff(y)) as isize;
            intersections.push(((sensor.0 - reach), (sensor.0 + reach)));
        }
    }
    intersections.sort();
    sorted_to_disjoint_intervals(&intersections)
}
fn count_impossible_beacons_in_row(sensor_beacons: &Vec<(Coord, Coord)>, y: isize) -> usize {
    let intersections = disjoint_intersections_with_row(sensor_beacons, y);

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
fn _part2_brute_force(sensor_beacons: &Vec<(Coord, Coord)>, min: isize, max: isize) -> isize {
    for y in min..=max {
        let intersections = disjoint_intersections_with_row(sensor_beacons, y);
        if intersections.len() > 1 {
            return (intersections[0].1 + 1) * max + y;
        }
    }
    0
}

// At a high level, we iterate through the sensors and find the unique point
// which is exactly one outside the range of four sensors. This will find
// the hole in our coverage
fn part2_fast(sensor_beacons: &Vec<(Coord, Coord)>, min: isize, max: isize) -> Option<isize> {
    sensor_beacons
        .iter()
        // First we map the sensors and beacons to Sensors and Ranges. We add 1 to the range to get
        // the minimum distance that the target and the mystery beacon
        .map(|(sensor, beacon)| (sensor, 1 + d(sensor, beacon)))
        // Next we look at all pairs of Sensors
        .combinations(2)
        // We find pairs of sensors which could both be just missing the beacon by 1.
        // since we are told the missing beacon is _unique_ in the search space,
        // we must have some beacons that miss it by exactly one on either side.
        //
        // this may be easier to picture if you rotate the grid 45 degrees mentally,
        // if you have a unique point not covered by rectangles, then there must be two
        // pairs of rectangles, one pair north south, another east west, which just miss
        // the beacon by one
        .filter_map(|sensors| {
            let (sensor1, range1) = sensors[0];
            let (sensor2, range2) = sensors[1];
            (d(sensor1, sensor2) == range1 + range2)
                .then_some(((sensor1, range1 as isize), (sensor2, range2 as isize)))
        })
        // We now have a list of sensor pairs that just miss. We need two such pairs
        // to uniquely specify coordinates. With the AoC inputs, we are almost guaranteed
        // that there are only two items in the above iterator, but in general there may
        // be more beacons that barely miss each other in the same way, so to be safe we
        // operate on all pairs of pairs. We call this a "cross", since if you join the
        // pairs with a straight line it will form an "X" shape
        .combinations(2)
        .filter_map(|cross| {
            // For a given pair, we need the arms of the cross to be at different angles.
            // an easy way to check this is to partition the cross arms, and check that one
            // of them has (x1 < x2) and (y1 < y2), and the other has (x1 < x2) and (y1 > y2).
            // the reason we use an XOR is to conveniently also cover the other two cases, where
            // one of our cross arms is in the other direction. If you do the maths, it turns out
            // that the ordering in a pair of sensors doesn't affect the formula
            let pair: (Vec<_>, Vec<_>) = cross
                .into_iter()
                .partition(|((Coord(x1, y1), _), (Coord(x2, y2), _))| (x1 < x2) ^ (y1 < y2));

            // if we don't have one pair going northeast and another pair going southwest, we don't have a cross
            // so we don't uniquely specify a point, so we can skip this case
            if let (Some(northwest), Some(southwest)) = (pair.0.get(0), pair.1.get(0)) {
                // the north-east to south-west cross arm will give us a constraint on our solution in the form x - y = const
                // the maths here isn't too tricky, although the final formula looks daunting. Assume x1 < x2 and y1 > y2
                // then for our solution (x, y) (with x1 < x < x2 and y1 < y < y2), we require
                // (x - x1) + (y1 - y) = d1 (the range of sensor1, plus 1)
                // (x2 - x) + (y - y2) = d2 (the range of sensor2, plus 1)
                // the equations end up the same for the x2 > x1 and y1 < y2, i.e the south-west to north-east case
                let ((Coord(x1, y1), d1), (Coord(x2, y2), d2)) = southwest;
                let x_plus_y = (d1 * (x2 + y2) + d2 * (x1 + y1)) / (d1 + d2);

                //similarly, for the south-east to north-west case, i.e for x1 < x < x2 and y1 < y < y2
                // (x - x1) + (y - y1) = d1
                // (x2 - x) + (y2 - y) = d2
                // and again, by symmetry, the case is the same if x2 < x < x1 and  y2 < y < y1
                let ((Coord(x1, y1), d1), (Coord(x2, y2), d2)) = northwest;
                let x_sub_y = (d1 * (x2 - y2) + d2 * (x1 - y1)) / (d1 + d2);

                // finally we transform our two equations giving x + y and x - y into their x and y coordinates
                Some(((x_plus_y + x_sub_y) / 2, (x_plus_y - x_sub_y) / 2))
            } else {
                None
            }
        })
        // at this stage, we should only have one solution, however it's possible that
        // we found another one outside the search space, so we filter by the bounds quickly
        .filter(|&(x, y)| x >= min && x <= max && y >= min && y <= max)
        // then we map our solution to the required one for day15
        .map(|(x, y)| x * max + y)
        // take the first solution, if it exists. If the problem is well defined, there should only be one
        //solution anyway
        .next()
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("inputs/day15.txt")?;

    let (remaining, sensor_beacons) = parse(&input).unwrap();
    assert_eq!(remaining.trim(), "");

    let part1 = count_impossible_beacons_in_row(&sensor_beacons, 2_000_000);
    println!("15.1: {part1}");

    println!("15.2: {:?}", part2_fast(&sensor_beacons, 0, 4_000_000));
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

    #[test]
    fn test_part2() {
        let input = std::fs::read_to_string("test_inputs/day15.txt").unwrap();

        let (remaining, sensor_beacons) = parse(&input).unwrap();

        assert_eq!(
            Some(_part2_brute_force(&sensor_beacons, 0, 4_000_000)),
            part2_fast(&sensor_beacons, 0, 4_000_000)
        );
    }
}
