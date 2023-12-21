use std::collections::HashMap;

use nom::character::complete::line_ending;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, multispace1},
    combinator::{map, map_res},
    multi::{count, separated_list1},
    sequence::{preceded, tuple},
    IResult,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Material {
    Ore = 0,
    Clay = 1,
    Obsidian = 2,
    Geode = 3,
}
#[derive(PartialEq, Eq, Debug, Clone)]
struct Recipe {
    creates: Material,
    costs: Vec<(usize, Material)>,
}

#[derive(Debug, Clone)]
struct Blueprint {
    recipes: Vec<Recipe>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct State {
    robots: [usize; 4],
    resources: [usize; 4],
    time: usize,
}

impl State {
    fn with_time(time: usize) -> Self {
        Self {
            robots: [1, 0, 0, 0],
            resources: [0, 0, 0, 0],
            time,
        }
    }
    fn finish(&mut self) {
        while self.time > 0 {
            *self = self.tick();
        }
    }
    fn tick(&self) -> Self {
        Self {
            robots: self.robots,
            resources: [0, 1, 2, 3].map(|i| self.resources[i] + self.robots[i]),
            time: self.time - 1,
        }
    }

    fn try_recipe(&self, r: &Recipe) -> Option<Self> {
        let mut state = self.clone();
        for &(cost, material) in &r.costs {
            state.resources[material as usize] =
                state.resources[material as usize].checked_sub(cost)?;
        }
        state.robots[r.creates as usize] += 1;
        state.resources = [0, 1, 2, 3].map(|i| state.resources[i] + self.robots[i]); //use old robots
        state.time -= 1;
        Some(state)
    }
}

fn material(input: &str) -> IResult<&str, Material> {
    map(
        alt((tag("ore"), tag("clay"), tag("obsidian"), tag("geode"))),
        |s| match s {
            "ore" => Material::Ore,
            "clay" => Material::Clay,
            "obsidian" => Material::Obsidian,
            "geode" => Material::Geode,
            _ => unreachable!(),
        },
    )(input)
}
fn cost(input: &str) -> IResult<&str, (usize, Material)> {
    tuple((
        map_res(digit1, str::parse::<usize>),
        preceded(multispace1, material),
    ))(input)
}

fn recipe(input: &str) -> IResult<&str, Recipe> {
    map(
        tuple((
            tag("Each "),
            material,
            tag(" robot costs "),
            separated_list1(tag(" and "), cost),
            tag("."),
        )),
        |(_, creates, _, costs, _)| Recipe { creates, costs },
    )(input)
}

fn blueprint(input: &str) -> IResult<&str, Blueprint> {
    map(
        preceded(
            tuple((tag("Blueprint "), digit1, tag(":"), multispace1)),
            separated_list1(multispace1, recipe),
        ),
        |recipes| Blueprint { recipes },
    )(input)
}

fn blueprints(input: &str) -> IResult<&str, Vec<Blueprint>> {
    separated_list1(multispace1, blueprint)(input)
}

fn most_geodes_produced(
    state: State,
    blueprint: &Blueprint,
    memo: &mut HashMap<State, usize>,
) -> usize {
    if state.time == 0 {
        return state.resources[3];
    }
    if let Some(result) = memo.get(&state) {
        return *result;
    }

    let mut max = 0;
    for recipe in &blueprint.recipes {
        if let Some(new_state) = state.try_recipe(recipe) {
            max = max.max(most_geodes_produced(new_state, blueprint, memo));
        }
    }
    max = max.max(most_geodes_produced(state.tick(), blueprint, memo));
    memo.insert(state, max);
    max
}

fn main() {
    let input = std::fs::read_to_string("inputs/day19.txt").unwrap();
    let (remaining, blueprints) = blueprints(&input).unwrap();
    assert_eq!(remaining.trim(), "");

    let mut total_quality = 0;
    for (i, blueprint) in blueprints.iter().enumerate() {
        let quality =
            (i + 1) * most_geodes_produced(State::with_time(24), &blueprint, &mut HashMap::new());
        total_quality += quality;
        println!("{quality}");
    }
    println!("19.1: {total_quality}");

    let mut total_quality = 0;
    for blueprint in blueprints.iter().take(3) {
        let quality = most_geodes_produced(State::with_time(32), &blueprint, &mut HashMap::new());
        total_quality *= quality;
        println!("{quality}");
    }
    println!("19.2: {total_quality}");
}
