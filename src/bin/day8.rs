use std::fs;

/// Traverses the forest exactly once
/// uses two pointers that converge for the left and right trees
/// uses a [u8; N] for the tallest tree visible in column i from the top
/// uses the lower 10 bits of a [u16; N] to store the trees visible from the bottom in columnn i
fn part1<const N: usize>(rows: &[&[u8]]) -> u32 {
    let mut bottom = [0; N]; //bitmask for trees visible from the bottom
    let mut top = [0u8; N]; //max values visible from above
    let [mut front_ptr, mut back_ptr]: [usize; 2];
    let [mut front, mut current, mut back]: [u8; 3];
    let mut include: bool;
    let mut result = 0;

    for row in rows {
        (front_ptr, back_ptr, front, back) = (0, N - 1, b'0' - 1, b'0' - 1);

        // move the pointers together till they kiss
        for _ in 0..N {
            include = false;
            if front <= back {
                current = row[front_ptr];

                //front check
                if current > front {
                    include = true;
                    front = current;
                }

                //top check
                if current > top[front_ptr] {
                    include = true;
                    top[front_ptr] = current;
                }

                //bottom check
                bottom[front_ptr] &= !(1u16) << (current - b'0'); //zero out all trees at or below current height
                bottom[front_ptr] |= (!include as u16) << (current - b'0'); //if !include, add back in tree at current height

                front_ptr += 1;
            } else {
                // same logic but for back of list
                current = row[back_ptr];
                if current > back {
                    include = true;
                    back = current;
                }
                if current > top[back_ptr] {
                    include = true;
                    top[back_ptr] = current;
                }

                bottom[back_ptr] &= !(1u16) << (current - b'0');
                bottom[back_ptr] |= (!include as u16) << (current - b'0');
                back_ptr -= 1;
            }

            if include {
                result += 1;
            }
        }
    }
    bottom.into_iter().map(|x| x.count_ones()).sum::<u32>() + result
}

fn scenic_score(rows: &[&[u8]], (start_x, start_y): (isize, isize)) -> isize {
    let height = rows[start_y as usize][start_x as usize];

    [(1, 0), (-1, 0), (0, 1), (0, -1)]
        .into_iter()
        .map(|(dx, dy)| {
            let (mut x, mut y) = (start_x + dx, start_y + dy);
            while let Some(&tree) = rows.get(y as usize).and_then(|row| row.get(x as usize)) {
                (x, y) = (x + dx, y + dy);
                if tree >= height {
                    break;
                }
            }
            (x.abs_diff(start_x) + y.abs_diff(start_y) - 1) as isize
        })
        .product()
}

fn part2(rows: &[&[u8]]) -> Option<isize> {
    (0..rows.len())
        .flat_map(|y| (0..rows[y].len()).map(move |x| scenic_score(rows, (x as isize, y as isize))))
        .max()
}
fn main() {
    let input: Vec<u8> = fs::read("inputs/day8.txt")
        .unwrap()
        .into_iter()
        .filter(|x| *x != b'\r')
        .collect();
    let lines: Vec<&[u8]> = input.split(|&x| x == b'\n').collect();

    let time = std::time::Instant::now();
    println!("8.1: {}", part1::<99>(&lines));
    println!("{:?}", time.elapsed());
    println!("8.2: {}", part2(&lines).unwrap());
    println!("{:?}", time.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input: Vec<u8> = fs::read("test_inputs/day8.txt")
            .unwrap()
            .into_iter()
            .filter(|x| *x != b'\r')
            .collect();
        let lines: Vec<&[u8]> = input.split(|&x| x == b'\n').collect();

        assert_eq!(21, part1::<5>(&lines));
    }

    #[test]
    fn test_part2() {
        let input: Vec<u8> = fs::read("test_inputs/day8.txt")
            .unwrap()
            .into_iter()
            .filter(|x| *x != b'\r')
            .collect();
        let lines: Vec<&[u8]> = input.split(|&x| x == b'\n').collect();

        assert_eq!(Some(8), part2(&lines));
    }
}
