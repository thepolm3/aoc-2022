use anyhow::Result;
use itertools::chain;
use itertools::Itertools;
use std::convert::identity;
use std::fs;

fn head(input: &str) -> Vec<(isize, isize)> {
    chain![
        [(0, 0)],
        input
            .lines()
            .map(|line| {
                line.split_once(" ")
                    .map(|(a, b)| (a, b.parse::<usize>().expect("parse error digit")))
                    .expect("bad line")
            })
            .scan((0_isize, 0_isize), |position, (dir, length)| {
                Some(
                    (0..length)
                        .map(|_| {
                            match dir {
                                "L" => position.0 -= 1,
                                "R" => position.0 += 1,
                                "D" => position.1 -= 1,
                                "U" => position.1 += 1,
                                _ => panic!("parse error LRUD"),
                            };
                            *position
                        })
                        .collect_vec(),
                )
            })
            .flatten() //head positions
    ]
    .collect()
}

fn tail(head: impl IntoIterator<Item = (isize, isize)>) -> Vec<(isize, isize)> {
    chain![
        [(0, 0)],
        head.into_iter()
            .scan((0, 0), |(t_x, t_y), h_position @ (h_x, h_y)| {
                Some(if h_x.abs_diff(*t_x).max(h_y.abs_diff(*t_y)) > 1 {
                    *t_x += h_x.cmp(t_x) as isize;
                    *t_y += h_y.cmp(t_y) as isize;
                    Some((*t_x, *t_y))
                } else {
                    None
                })
            })
            .filter_map(identity)
    ]
    .collect()
}
fn part1(input: &str) -> usize {
    tail(head(input)).into_iter().unique().count()
}
fn part2(input: &str, length: usize) -> usize {
    let mut link = head(input);
    for _ in 1..length {
        link = tail(link);
    }
    link.into_iter().unique().count()
}
fn main() -> Result<()> {
    let input = fs::read_to_string("inputs/day9.txt")?;

    println!("9.1 {}", part1(&input));
    println!("9.2 {}", part2(&input, 10));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("test_inputs/day9.txt").unwrap();

        assert_eq!(part1(&input), 13);
    }
    #[test]
    fn test_part2() {
        let input = fs::read_to_string("test_inputs/day9.txt").unwrap();

        assert_eq!(part2(&input, 10), 1);
        let input = fs::read_to_string("test_inputs/day9-2.txt").unwrap();

        assert_eq!(part2(&input, 36), 1);
    }
}
