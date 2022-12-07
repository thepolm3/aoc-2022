use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fmt::Display;
use std::fs;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, line_ending, not_line_ending},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

#[derive(Debug, PartialEq, Eq)]
enum ConsoleLine {
    Cd(String),
    Ls,
    File(File),
    Directory(Directory),
}

#[derive(Debug, PartialEq, Eq)]
struct File {
    size: usize,
    name: String,
}
impl Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "- {} ({})", self.name, self.size)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Directory {
    name: String,
    files: HashMap<String, File>,
    subdirectories: HashMap<String, Directory>,
}
impl Display for Directory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "- {}\n  {}\n{}",
            self.name,
            self.files
                .iter()
                .map(|(_, x)| format!("{}", x))
                .collect::<Vec<String>>()
                .join("\n  "),
            self.subdirectories
                .iter()
                .map(|(_, x)| format!("{}", x)
                    .lines()
                    .map(|x| x.into())
                    .collect::<Vec<String>>())
                .flatten()
                .collect::<Vec<String>>()
                .join("\n  "),
        )
    }
}
impl Directory {
    fn new(name: String) -> Self {
        Self {
            name,
            files: HashMap::new(),
            subdirectories: HashMap::new(),
        }
    }

    fn merge(&mut self, other: Self) {
        self.files.extend(other.files);
        self.subdirectories.extend(other.subdirectories);
    }

    fn size(&self) -> usize {
        self.files.values().map(|f| f.size).sum::<usize>()
            + self
                .subdirectories
                .values()
                .map(|d| d.size())
                .sum::<usize>()
    }

    fn walk_dirs(&self) -> Vec<&Directory> {
        Iterator::chain(
            std::iter::once(self),
            self.subdirectories
                .values()
                .map(|d| d.walk_dirs())
                .flatten(),
        )
        .collect()
    }
}

fn parse_instruction(input: &str) -> IResult<&str, ConsoleLine> {
    alt((
        map(preceded(tag("$ cd "), not_line_ending), |path: &str| {
            ConsoleLine::Cd(path.into())
        }),
        map(tag("$ ls"), |_| ConsoleLine::Ls),
        map(preceded(tag("dir "), not_line_ending), |name: &str| {
            ConsoleLine::Directory(Directory::new(name.into()))
        }),
        map(
            separated_pair(digit1::<&str, _>, tag(" "), not_line_ending),
            |(size, name)| {
                ConsoleLine::File(File {
                    size: size.parse().unwrap(),
                    name: name.to_owned(),
                })
            },
        ),
    ))(input)
}

fn parse_instruction_list(input: &str) -> IResult<&str, Vec<ConsoleLine>> {
    separated_list1(line_ending, parse_instruction)(input)
}

fn build_directory_tree(input: Vec<ConsoleLine>) -> Result<Directory> {
    let mut stack: Vec<Directory> = vec![];
    for line in input {
        match line {
            ConsoleLine::Cd(path) => {
                if path == ".." {
                    let last = stack.pop().context("cannot go up directory")?;
                    let parent_dir = stack
                        .last_mut()
                        .context("stack empty")?
                        .subdirectories
                        .entry(last.name.clone())
                        .or_insert(Directory::new(last.name.clone()));
                    parent_dir.merge(last);
                } else {
                    let entry = stack
                        .last_mut()
                        .unwrap_or(&mut Directory::new(path.clone()))
                        .subdirectories
                        .remove(&path)
                        .unwrap_or(Directory::new(path));
                    stack.push(entry);
                }
            }
            ConsoleLine::Ls => {}
            ConsoleLine::File(file) => {
                stack
                    .last_mut()
                    .context("no directory tree to add file to")?
                    .files
                    .insert(file.name.clone(), file);
            }
            ConsoleLine::Directory(dir) => {
                stack
                    .last_mut()
                    .context("no directory tree to add file to")?
                    .subdirectories
                    .insert(dir.name.clone(), dir);
            }
        }
    }
    while stack.len() > 1 {
        let last = stack.pop().unwrap();
        stack
            .last_mut()
            .unwrap()
            .subdirectories
            .insert(last.name.clone(), last);
    }
    stack.pop().context("Empty instructions")
}

fn part1(dir: &Directory) -> usize {
    dir.walk_dirs()
        .iter()
        .map(|d| d.size())
        .filter(|size| *size <= 100_000)
        .sum()
}

fn part2(dir: &Directory) -> Option<usize> {
    let target_size = dir.size().checked_sub(40_000_000)?;
    dir.walk_dirs()
        .iter()
        .map(|d| d.size())
        .filter(|size| *size >= target_size)
        .min()
}
fn main() -> Result<()> {
    let input = fs::read_to_string("inputs/day7.txt").unwrap();
    let instructions = parse_instruction_list(&input).unwrap().1;
    let dtree = build_directory_tree(instructions)?;

    println!("7.1: {}", part1(&dtree));
    println!("7.2: {}", part2(&dtree).unwrap());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        let input = fs::read_to_string("test_inputs/day7.txt").unwrap();
        let instructions = parse_instruction_list(&input).unwrap().1;
        let dtree = build_directory_tree(instructions).unwrap();

        assert_eq!(part1(&dtree), 95437);
    }

    #[test]
    fn test_part2() {
        let input = fs::read_to_string("test_inputs/day7.txt").unwrap();
        let instructions = parse_instruction_list(&input).unwrap().1;
        let dtree = build_directory_tree(instructions).unwrap();

        assert_eq!(part2(&dtree).unwrap(), 24933642);
    }
}
