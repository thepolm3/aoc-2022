use anyhow::{Context, Result};
use itertools::Itertools;

#[derive(Debug, Eq, PartialEq)]
struct GridSquare {
    height: u8,
    distance: Option<usize>,
}
impl GridSquare {
    fn new(height: u8) -> Self {
        Self {
            height,
            distance: None,
        }
    }
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("inputs/day12.txt")?;
    let w = input.lines().next().context("No input")?.chars().count();
    let start = input
        .chars()
        .filter(|c| !c.is_ascii_whitespace())
        .position(|x| x == 'S')
        .context("No start")?;
    let end = input
        .chars()
        .filter(|c| !c.is_ascii_whitespace())
        .position(|x| x == 'E')
        .context("No end")?;
    let mut grid: Vec<_> = input
        .chars()
        .filter(|c| !c.is_ascii_whitespace())
        .enumerate()
        .map(|(_, c)| {
            GridSquare::new(match c {
                'S' => 0,
                'E' => 25,
                c => c as u8 - b'a',
            })
        })
        .collect();

    let mut to_search: Vec<usize>;
    let mut next_search = vec![end];
    let mut part2 = Some(0);

    grid[end].distance = Some(0);
    'outer: for i in 0_usize.. {
        to_search = next_search.into_iter().unique().collect();
        next_search = Vec::new();
        if to_search.is_empty() {
            break 'outer;
        }
        while let Some(cur) = to_search.pop() {
            grid[cur].distance = Some(i);
            if cur == start {
                break 'outer;
            }
            if grid[cur].height == 0 {
                part2 = grid[cur].distance;
            }
            next_search.extend(
                [cur.wrapping_sub(1), cur + 1, cur.wrapping_sub(w), cur + w]
                    .into_iter()
                    .filter_map(|nbr| match grid.get(nbr).and_then(|x| x.distance) {
                        Some(_) => None,
                        None => grid.get(cur).and_then(|to| {
                            grid.get(nbr).and_then(|from| {
                                (to.height.saturating_sub(from.height) <= 1).then_some(nbr)
                            })
                        }),
                    }),
            );
        }
    }
    println!(
        "12.1 {:?}",
        grid[start].distance.context("no solution for part 1")?
    );
    println!("12.2 {:?}", part2.context("no solution for part 2")?);

    Ok(())
}
