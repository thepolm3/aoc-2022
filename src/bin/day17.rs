use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    iter::{repeat, Cycle},
};

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::line_ending,
    combinator::{map, opt},
    multi::{count, many1, many_till, separated_list1},
    sequence::tuple,
    IResult,
};

const SHAPES: &str = include_str!("day17shapes.txt");

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
enum Motion {
    Left,
    Right,
}
impl Display for Motion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Motion::Left => "<",
                Motion::Right => ">",
            }
        )
    }
}
impl Motion {
    fn from_str(s: &str) -> Self {
        match s {
            ">" => Self::Right,
            "<" => Self::Left,
            _ => panic!("Invalid string passed to Motion::from_str"),
        }
    }
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum State {
    Rock,
    Air,
}
impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                State::Rock => "#",
                State::Air => ".",
            }
        )
    }
}
impl State {
    fn from_str(s: &str) -> Self {
        match s {
            "#" => Self::Rock,
            "." => Self::Air,
            _ => panic!("Invalid string passed to Motion::from_str"),
        }
    }

    fn is_solid(&self) -> bool {
        self == &Self::Rock
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct TetrisPiece {
    width: usize,
    height: usize,
    inner: Vec<State>,
}
impl Display for TetrisPiece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.inner[..].chunks_exact(self.width) {
            write!(f, "{}\n", row.iter().map(|s| format!("{}", s)).join(""))?;
        }
        Ok(())
    }
}
impl TetrisPiece {
    fn new(width: usize, inner: Vec<State>) -> TetrisPiece {
        Self {
            width,
            height: inner.len() / width,
            inner,
        }
    }

    fn enumerate_points(
        &self,
        (x, y): (usize, usize),
    ) -> impl Iterator<Item = ((usize, usize), &State)> {
        self.inner
            .iter()
            .enumerate()
            .map(move |(i, s)| ((x + i % self.width, y + i / self.width), s))
    }

    fn into_enumerate_points(
        self,
        (x, y): (usize, usize),
    ) -> impl Iterator<Item = ((usize, usize), State)> {
        self.inner
            .into_iter()
            .enumerate()
            .map(move |(i, s)| ((x + i % self.width, y + i / self.width), s))
    }
}

struct PlayGrid {
    width: usize,
    inner: Vec<State>,
}
impl Display for PlayGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.inner[..].chunks_exact(self.width).rev().take(50) {
            write!(f, "|{}|\n", row.iter().map(|s| format!("{}", s)).join(""))?;
        }
        write!(f, "+{}+", "-".repeat(self.width))
    }
}

impl PlayGrid {
    fn new(width: usize) -> Self {
        Self {
            width,
            inner: Vec::new(),
        }
    }

    fn height(&self) -> usize {
        self.inner.len() / self.width
    }

    fn get(&self, (x, y): (usize, usize)) -> Option<&State> {
        if x >= self.width {
            return None;
        }
        if y >= self.height() {
            return Some(&State::Air);
        }

        Some(&self.inner[y * self.width + x])
    }

    fn set(&mut self, (x, y): (usize, usize), state: State) {
        if x >= self.width {
            return;
        }
        if y >= self.height() {
            self.inner
                .extend(repeat(State::Air).take((y + 1) * self.width - self.inner.len()));
        }
        self.inner[y * self.width + x] = state;
    }

    fn set_tetris_piece(&mut self, position: (usize, usize), piece: TetrisPiece) {
        for (position, state) in piece.into_enumerate_points(position) {
            if state.is_solid() {
                self.set(position, state);
            }
        }
    }

    fn collides_with_tetris_piece(&self, shape: &TetrisPiece, position: (usize, usize)) -> bool {
        for ((x, y), state) in shape.enumerate_points(position) {
            if state == &State::Air {
                continue;
            }
            if self.get((x, y)).map(State::is_solid).unwrap_or(true) {
                return true;
            }
        }
        false
    }
}

fn one_move(input: &str) -> IResult<&str, Motion> {
    map(alt((tag("<"), tag(">"))), Motion::from_str)(input)
}

fn parse_moves(input: &str) -> IResult<&str, Vec<Motion>> {
    many1(one_move)(input)
}

fn one_state(input: &str) -> IResult<&str, State> {
    map(alt((tag("."), tag("#"))), State::from_str)(input)
}

fn parse_shape(input: &str) -> IResult<&str, TetrisPiece> {
    let (remaining, rows) = separated_list1(line_ending, many1(one_state))(input)?;
    Ok((
        remaining,
        TetrisPiece::new(
            rows[0].len(),
            rows.into_iter().rev().flat_map(|x| x.into_iter()).collect(),
        ),
    ))
}

fn parse_shapes(input: &str) -> IResult<&str, Vec<TetrisPiece>> {
    separated_list1(count(line_ending, 2), parse_shape)(input)
}

fn part1(moves: &Vec<Motion>, shapes: &Vec<TetrisPiece>) -> usize {
    let mut grid = PlayGrid::new(7);
    let mut moves = moves.into_iter().cycle();
    for shape in shapes.into_iter().cycle().take(2022) {
        let mut position: (usize, usize) = (2, grid.height() + 3);
        while let Some(mv) = moves.next() {
            let new_position = (
                match mv {
                    Motion::Left => position.0.saturating_sub(1),
                    Motion::Right => position.0 + 1,
                },
                position.1,
            );
            if !grid.collides_with_tetris_piece(&shape, new_position) {
                position = new_position;
            }
            if position.1 == 0
                || grid.collides_with_tetris_piece(&shape, (position.0, position.1 - 1))
            {
                grid.set_tetris_piece(position, shape.clone());
                break;
            } else {
                position.1 -= 1;
            }
        }
    }
    grid.height()
}

fn detect_cycle(moves: &Vec<Motion>, shapes: &Vec<TetrisPiece>, iterations: usize) -> usize {
    let mut seen = HashMap::new();
    let mut grid = PlayGrid::new(7);
    let mut i = 0;
    let mut skipped_height = 0;
    let mut mv_idx = 0;
    let mut cycle_detected = false;
    while let Some(shape) = shapes.get(i % shapes.len()) {
        if !cycle_detected {
            let last_seen = seen.insert(
                (i % shapes.len(), mv_idx, format!("{}", grid)),
                (i, grid.height()),
            );
            if let Some((j, h)) = last_seen {
                skipped_height =
                    h + (grid.height() - h) * ((iterations - j) / (i - j)) - grid.height();
                i = j + (i - j) * ((iterations - j) / (i - j));
                cycle_detected = true;
                println!("{i}");
            };
        }
        let mut position: (usize, usize) = (2, grid.height() + 3);
        while let Some(mv) = moves.get(mv_idx) {
            mv_idx = (mv_idx + 1) % moves.len();
            let new_position = (
                match mv {
                    Motion::Left => position.0.saturating_sub(1),
                    Motion::Right => position.0 + 1,
                },
                position.1,
            );
            if !grid.collides_with_tetris_piece(&shape, new_position) {
                position = new_position;
            }
            if position.1 == 0
                || grid.collides_with_tetris_piece(&shape, (position.0, position.1 - 1))
            {
                grid.set_tetris_piece(position, shape.clone());
                break;
            } else {
                position.1 -= 1;
            }
        }
        if i == iterations - 1 {
            break;
        }
        i += 1;
    }
    grid.height() + skipped_height
}

fn main() {
    let input = std::fs::read_to_string("inputs/day17.txt").unwrap();
    let (remaining, shapes) = parse_shapes(SHAPES).unwrap();
    assert_eq!(remaining.trim(), "");

    let (remaining, moves) = parse_moves(&input).unwrap();
    assert_eq!(remaining.trim(), "");

    println!("{}", part1(&moves, &shapes));
    println!("17.1: {}", detect_cycle(&moves, &shapes, 2022));
    println!("17.2: {}", detect_cycle(&moves, &shapes, 1000000000000));
}
