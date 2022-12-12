use anyhow::{Context, Result};

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
    let mut part2 = None;

    let now = std::time::Instant::now();
    grid[end].distance = Some(0);
    'outer: for i in 1_usize.. {
        (next_search, to_search) = (Vec::new(), next_search);
        if to_search.is_empty() {
            break 'outer;
        }
        while let Some(cur) = to_search.pop() {
            if cur == start {
                break 'outer;
            }
            if grid[cur].height == 0 && part2.is_none() {
                part2 = grid[cur].distance;
            }
            for nbr in [cur.wrapping_sub(1), cur + 1, cur.wrapping_sub(w), cur + w] {
                if nbr >= grid.len() {
                    continue;
                }
                if let (Some(from), Some(to)) = match nbr < cur {
                    true => {
                        let split_grid = grid.split_at_mut(cur);
                        (split_grid.0.get_mut(nbr), split_grid.1.get_mut(0))
                    }
                    false => {
                        let split_grid = grid.split_at_mut(nbr);
                        (split_grid.1.get_mut(0), split_grid.0.get_mut(cur))
                    }
                } {
                    if from.distance.is_some() {
                        continue;
                    };
                    if to.height.saturating_sub(from.height) > 1 {
                        continue;
                    };
                    from.distance = Some(i);
                    next_search.push(nbr);
                };
            }
        }
    }
    println!("12.1 {:?}", grid[start].distance.context("no solution p1")?);
    println!("12.2 {:?}", part2.context("no solution p2")?);
    println!("{:?}", now.elapsed());

    Ok(())
}
