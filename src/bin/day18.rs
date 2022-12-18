use std::collections::{BTreeSet, HashSet, VecDeque};

use anyhow::Result;
use itertools::Itertools;

fn nbrs([x, y, z]: [i32; 3]) -> [[i32; 3]; 6] {
    [
        [x + 1, y, z],
        [x, y + 1, z],
        [x, y, z + 1],
        [x.wrapping_sub(1), y, z],
        [x, y.wrapping_sub(1), z],
        [x, y, z.wrapping_sub(1)],
    ]
}
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
enum Facing {
    X = 0,
    Y = 1,
    Z = 2,
}
impl Facing {
    fn next(&self) -> Self {
        Self::from_usize(((*self as usize) + 1) % 3)
    }

    fn prev(&self) -> Self {
        Self::from_usize(((*self as usize) + 2) % 3)
    }

    fn from_usize(from: usize) -> Self {
        match from {
            0 => Self::X,
            1 => Self::Y,
            2 => Self::Z,
            _ => panic!("usize out of range"),
        }
    }
    fn step_forward(&self, mut position: [i32; 3], n: i32) -> [i32; 3] {
        position[*self as usize] += n;
        position
    }
    fn step_back(&self, mut position: [i32; 3], n: i32) -> [i32; 3] {
        self.step_forward(position, -n)
    }
}

fn faces([x, y, z]: [i32; 3]) -> [([i32; 3], Facing); 6] {
    [
        ([x + 1, y, z], Facing::X),
        ([x, y + 1, z], Facing::Y),
        ([x, y, z + 1], Facing::Z),
        ([x, y, z], Facing::X),
        ([x, y, z], Facing::Y),
        ([x, y, z], Facing::Z),
    ]
}
// //returns face neigbors in pairs, if the first face is present you can ignore the 2nd
// fn face_nbrs(([x, y, z], facing): ([usize; 3], Facing)) -> [[([usize; 3], Facing); 3]; 4] {
//     [
//         [
//             ([x, y, z], facing.next()),
//             (facing.next().step_back([x, y, z], 1), facing),
//             (facing.step_back([x, y, z], 1), facing.next()),
//         ],
//         [
//             ([x, y, z], facing.prev()),
//             (facing.prev().step_back([x, y, z], 1), facing),
//             (facing.step_back([x, y, z], 1), facing.prev()),
//         ],
//         [
//             (facing.next().step_forward([x, y, z], 1), facing.next()),
//             (facing.next().step_forward([x, y, z], 1), facing),
//             (
//                 facing.step_back(facing.next().step_forward([x, y, z], 1), 1),
//                 facing.next(),
//             ),
//         ],
//         [
//             (facing.prev().step_forward([x, y, z], 1), facing.prev()),
//             (facing.prev().step_forward([x, y, z], 1), facing),
//             (
//                 facing.step_back(facing.prev().step_forward([x, y, z], 1), 1),
//                 facing.prev(),
//             ),
//         ],
//     ]
// }

// ([x,y,z], X) means the border between ([x,y,z] and [x-1,y,z])

// fn part2(cubes: Vec<[usize; 3]>) {
//     let mut faces_set = BTreeSet::new();

//     for cube in cubes {
//         for face in faces(cube) {
//             if faces_set.contains(&face) {
//                 faces_set.remove(&face);
//             } else {
//                 faces_set.insert(face);
//             }
//         }
//     }
//     let mut regions = Vec::new();
//     while let Some(source_face) = faces_set.pop_first() {
//         let mut region = Vec::new();
//         let mut queue = VecDeque::from(vec![source_face]);
//         let reference_set = faces_set.clone();
//         while let Some(face) = queue.pop_front() {
//             for nbr_group in face_nbrs(face) {
//                 let mut result = None;
//                 for nbr in nbr_group {
//                     if reference_set.contains(&nbr) {
//                         result = faces_set.take(&nbr);
//                         break;
//                     }
//                 }
//                 if let Some(result) = result {
//                     queue.push_back(result);
//                 }
//             }
//             region.push(face);
//         }
//         regions.push(region);
//         println!(
//             "{:?}",
//             faces_set.len() + regions.iter().map(Vec::len).sum::<usize>()
//         );
//     }
//     println!("regions: {:?}", regions.len());
//     for region in &regions {
//         println!("{:?}", region.len());
//         println!("{:?}", region);
//     }
// }
fn part1(cubes: &[[i32; 3]]) -> i32 {
    let mut hs = HashSet::new();

    let mut total = 0;
    for cube in cubes {
        let mut contribution = 6;
        for nbr in nbrs(*cube) {
            if hs.contains(&nbr) {
                contribution -= 2;
            }
        }
        hs.insert(cube);
        total += contribution;
    }

    total
}

fn part2(cubes: &[[i32; 3]]) -> Option<i32> {
    let hs: HashSet<[i32; 3]> = HashSet::from_iter(cubes.to_owned());

    let (xmin, xmax) = cubes.iter().map(|cube| cube[0]).minmax().into_option()?;
    let (ymin, ymax) = cubes.iter().map(|cube| cube[1]).minmax().into_option()?;
    let (zmin, zmax) = cubes.iter().map(|cube| cube[2]).minmax().into_option()?;
    println!("{}", (xmax - xmin) * (ymax - ymin) * (zmax - zmin));
    let current_cube = [xmin - 1, ymin - 1, zmin - 1];
    let mut filled_region: HashSet<[i32; 3]> = HashSet::from_iter([current_cube]);
    let mut queue = VecDeque::from([current_cube]);
    while let Some(cube) = queue.pop_front() {
        for nbr @ [x, y, z] in nbrs(cube) {
            if x < xmin - 1
                || x > xmax + 1
                || y < ymin - 1
                || y > ymax + 1
                || z < zmin - 1
                || z > zmax + 1
            {
                continue;
            }
            if !hs.contains(&nbr) && !filled_region.contains(&nbr) {
                filled_region.insert(nbr);
                queue.push_back(nbr);
            }
        }
    }
    let [sidex, sidey, sidez] = [xmax - xmin + 3, ymax - ymin + 3, zmax - zmin + 3];

    Some(
        part1(&filled_region.into_iter().collect_vec())
            - 2 * (sidex * sidey + sidey * sidez + sidex * sidez),
    )
}
fn main() -> Result<()> {
    let cubes = std::fs::read_to_string("inputs/day18.txt")?
        .lines()
        .map(|l| -> Result<[i32; 3]> {
            let x = l.split(",").collect_vec();

            Ok([x[0].parse()?, x[1].parse()?, x[2].parse()?])
        })
        .collect::<Result<Vec<_>, _>>()?;

    // println!("18.1: {}", part1(&cubes));
    println!("18.1: {:?}", part2(&cubes));

    Ok(())
}
