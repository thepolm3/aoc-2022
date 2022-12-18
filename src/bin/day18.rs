use std::collections::HashSet;

use anyhow::Result;
use itertools::Itertools;

fn nbrs([x, y, z]: [usize; 3]) -> [[usize; 3]; 6] {
    [
        [x + 1, y, z],
        [x, y + 1, z],
        [x, y, z + 1],
        [x.wrapping_sub(1), y, z],
        [x, y.wrapping_sub(1), z],
        [x, y, z.wrapping_sub(1)],
    ]
}
fn main() -> Result<()> {
    let cubes = std::fs::read_to_string("inputs/day18.txt")?
        .lines()
        .map(|l| -> Result<[usize; 3]> {
            let x = l.split(",").collect_vec();

            Ok([x[0].parse()?, x[1].parse()?, x[2].parse()?])
        })
        .collect::<Result<Vec<_>, _>>()?;

    let mut hs = HashSet::new();

    let mut total = 0;
    for cube in cubes {
        let mut contribution = 6;
        for nbr in nbrs(cube) {
            if hs.contains(&nbr) {
                contribution -= 2;
            }
        }
        hs.insert(cube);
        total += contribution;
    }

    println!("{total}");

    Ok(())
}
