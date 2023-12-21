use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("inputs/day20.txt").unwrap();
    let sequence: Vec<_> = input
        .lines()
        .map(|l| l.trim().parse::<isize>().unwrap())
        .collect();

    let m = sequence.len();
    let mut mixed: Vec<usize> = (0..m).collect();
    for (i, item) in sequence.iter().enumerate() {
        let idx = mixed.iter().position(|&x| x == i).unwrap();
        let moved = mixed.remove(idx);
        mixed.insert(
            (idx as isize + item + 2 * m as isize - 3) as usize % (m - 1) + 1,
            moved,
        );
    }
    let mixed: Vec<isize> = mixed.into_iter().map(|x: usize| sequence[x]).collect();
    println!("{mixed:?}");

    let idx = mixed.iter().position(|&x| x == 0).unwrap();

    println!(
        "20.1 {}",
        (1..=3)
            .map(|i| mixed[(1000 * i + idx) % m as usize])
            .inspect(|x| println!("{x}"))
            .sum::<isize>()
    );
}
