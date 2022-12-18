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
    Shadow,
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
                Self::Shadow => "v",
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
                .map(|(ln, x)| format!("{:03} {}", ln, x))
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

    fn add_line_of(&mut self, from: (usize, usize), to: (usize, usize), gridsquare: GridSquare) {
        if from.0 == to.0 {
            for y in from.1.min(to.1)..=from.1.max(to.1) {
                self[(from.0, y)] = gridsquare;
            }
        } else if from.1 == to.1 {
            let xrange = (from.0.min(to.0), from.0.max(to.0));
            for x in xrange.0..=xrange.1 {
                self[(x, from.1)] = gridsquare;
            }
        } else {
            panic!("{:?} {:?}, line is not straight!", from, to);
        }
    }

    fn cast_shadows(&mut self) {
        for i in 1..self.ysize {
            let (start, rest) = self.inner.split_at_mut(i * self.xsize);
            let current_row = &start[(i - 1) * self.xsize..i * self.xsize];
            let next_row = &mut rest[1..self.xsize - 1];
            for (j, items) in current_row.windows(3).enumerate() {
                if items.iter().all(|x| !x.is_empty()) && next_row[j].is_empty() {
                    next_row[j] = GridSquare::Shadow;
                }
            }
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

fn sand_to_overflow(grid: &mut Grid) -> Result<usize, usize> {
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
    unreachable!(); //all grids either overflow or reach their starting point
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

    let mut grid = Grid::new(xrange, (0, yrange.1 + 1));
    for (from, to) in paths.iter().flat_map(|path| path.iter().tuple_windows()) {
        grid.add_line_of(*from, *to, GridSquare::Rock);
    }

    println!("14.1 {}", sand_to_overflow(&mut grid).unwrap());
    grid.cast_shadows();
    let part2 = grid.ysize.pow(2) - grid.inner.iter().filter(|x| !x.is_empty()).count();

    println!("14.2 {}", part2);

    //manual part2

    let mut grid = Grid::new(
        (
            xrange.0.min(500 - yrange.1 - 5),
            xrange.1.max(500 + yrange.1 + 5),
        ),
        (0, yrange.1 + 2),
    );

    for (from, to) in paths.iter().flat_map(|path| path.iter().tuple_windows()) {
        grid.add_line_of(*from, *to, GridSquare::Rock);
    }
    grid.add_line_of(
        (grid.xstart, yrange.1 + 2),
        (grid.xstart + grid.xsize - 1, yrange.1 + 2),
        GridSquare::Rock,
    );
    println!("14.2 {}", sand_to_overflow(&mut grid).unwrap_err());
    Ok(())
}
