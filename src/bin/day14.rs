use std::{
    fmt::Display,
    iter::repeat,
    ops::{Index, IndexMut},
};

use anyhow::{Context, Result};
use itertools::Itertools;
use nom::{
    bytes::complete::tag, character::complete::line_ending, combinator::map_res,
    multi::separated_list1, sequence::separated_pair, IResult,
};
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GridSquare {
    Empty,
    Rock,
    Sand,
}
impl GridSquare {
    fn is_empty(&self) -> bool {
        self == &Self::Empty
    }
}
impl Display for GridSquare {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Empty => " ",
                Self::Rock => "#",
                Self::Sand => "o",
            }
        )
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Grid {
    xstart: usize,
    ystart: usize,
    xsize: usize,
    ysize: usize,
    inner: Vec<GridSquare>,
}
impl Index<(usize, usize)> for Grid {
    type Output = GridSquare;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.inner[(index.1 - self.ystart) * self.xsize + index.0 - self.xstart]
    }
}
impl IndexMut<(usize, usize)> for Grid {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut GridSquare {
        &mut self.inner[(index.1 - self.ystart) * self.xsize + index.0 - self.xstart]
    }
}
impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.inner
                .chunks_exact(self.xsize)
                .map(|chunk| chunk.iter().map(|x| format!("{x}")).join(""))
                .enumerate()
                .map(|(ln, x)| format!("{} {}", ln, x))
                .join("\n")
        )
    }
}
impl Grid {
    fn new(xbounds: (usize, usize), ybounds: (usize, usize)) -> Self {
        Self {
            xstart: xbounds.0,
            ystart: ybounds.0,
            xsize: xbounds.1 - xbounds.0 + 1,
            ysize: ybounds.1 - ybounds.0 + 1,
            inner: Vec::from_iter(
                repeat(GridSquare::Empty)
                    .take((xbounds.1 - xbounds.0 + 1) * (ybounds.1 - ybounds.0 + 1)),
            ),
        }
    }

    fn get(&self, coords: (usize, usize)) -> Option<&GridSquare> {
        (self.xstart <= coords.0
            && self.ystart <= coords.1
            && coords.0 < self.xstart + self.xsize
            && coords.1 < self.ystart + self.ysize)
            .then(|| &self[coords])
    }

    fn add_rock(&mut self, from: (usize, usize), to: (usize, usize)) {
        let start = (from.0.min(to.0), from.1.min(to.1));
        let end = (from.0.max(to.0), from.1.max(to.1));
        for (x, y) in (start.0..=end.0).cartesian_product(start.1..=end.1) {
            self[(x, y)] = GridSquare::Rock;
        }
    }
}

fn digit1(input: &str) -> IResult<&str, usize> {
    map_res(nom::character::complete::digit1, str::parse)(input)
}

fn coordinate(input: &str) -> IResult<&str, (usize, usize)> {
    separated_pair(digit1, tag(","), digit1)(input)
}

fn path(input: &str) -> IResult<&str, Vec<(usize, usize)>> {
    separated_list1(tag(" -> "), coordinate)(input)
}

fn parse(input: &str) -> IResult<&str, Vec<Vec<(usize, usize)>>> {
    separated_list1(line_ending, path)(input)
}

fn sand_to_overflow(mut grid: Grid) -> Result<usize, usize> {
    let mut path = Vec::new();
    for i in 0.. {
        let (mut x, mut y) = path.pop().unwrap_or((500, 0));
        if !grid[(500, 0)].is_empty() {
            return Err(i);
        }
        'inner: loop {
            for coords in [(x, y + 1), (x - 1, y + 1), (x + 1, y + 1)] {
                if let Some(square) = grid.get(coords) {
                    if square.is_empty() {
                        path.push((x, y));
                        (x, y) = coords;
                        continue 'inner;
                    }
                } else {
                    return Ok(i);
                }
            }
            grid[(x, y)] = GridSquare::Sand;
            break 'inner;
        }
    }
    unreachable!();
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("inputs/day14.txt")?;
    let (remaining, paths) = parse(&input).unwrap();
    assert_eq!(remaining.trim(), "");
    let xrange = paths
        .iter()
        .flat_map(|path| path.iter().map(|(x, _)| *x))
        .minmax()
        .into_option()
        .context("No items")?;

    let yrange = paths
        .iter()
        .flat_map(|path| path.iter().map(|(_, y)| *y))
        .minmax()
        .into_option()
        .context("No items")?;

    let mut grid = Grid::new(xrange, (0, yrange.1));

    for (from, to) in paths.iter().flat_map(|path| path.iter().tuple_windows()) {
        grid.add_rock(*from, *to);
    }

    println!("14.1 {}", sand_to_overflow(grid).unwrap());

    let mut grid = Grid::new(
        (
            xrange.0.min(500 - (yrange.1 + 3)),
            xrange.1.max(500 + (yrange.1 + 3)),
        ),
        (0, yrange.1 + 2),
    );

    for (from, to) in paths.iter().flat_map(|path| path.iter().tuple_windows()) {
        grid.add_rock(*from, *to);
    }
    grid.add_rock(
        (grid.xstart, yrange.1 + 2),
        (grid.xstart + grid.xsize - 1, yrange.1 + 2),
    );

    println!("14.2 {}", sand_to_overflow(grid).unwrap_err());

    Ok(())
}
