fn mix(mut indexes: Vec<usize>, sequence: &[isize]) -> Vec<usize> {
    let m = sequence.len();
    for (i, &movement) in sequence.iter().enumerate() {
        let idx = indexes.iter().position(|&x| x == i).unwrap();
        let moved = indexes.remove(idx);
        indexes.insert(new_idx(idx, movement, m), moved);
    }
    indexes
}

fn new_idx(old_idx: usize, change: isize, modulus: usize) -> usize {
    let (old_idx, modulus) = (old_idx as isize, modulus as isize);
    (match change == 0 {
        true => old_idx,
        false => (old_idx + change - 1).rem_euclid(modulus - 1) + 1,
    }) as usize
}

fn grove_value(sequence: &[isize], n: usize) -> isize {
    let mut indexes: Vec<usize> = (0..sequence.len()).collect();
    for _ in 0..n {
        indexes = mix(indexes, sequence)
    }

    let mixed = lookup(indexes, sequence);

    let idx = mixed.iter().position(|&x| x == 0).unwrap();
    (1..=3)
        .map(|i| mixed[(1000 * i + idx) % mixed.len()])
        .sum::<isize>()
}

fn lookup(mixed: Vec<usize>, sequence: &[isize]) -> Vec<isize> {
    mixed.into_iter().map(|x| sequence[x]).collect()
}

fn main() {
    let input = std::fs::read_to_string("inputs/day20.txt").unwrap();
    let sequence: Vec<_> = input
        .lines()
        .map(|l| l.trim().parse::<isize>().unwrap())
        .collect();

    println!("20.1 {}", grove_value(&sequence, 1));

    let sequence: Vec<_> = sequence.into_iter().map(|x| x * 811589153).collect();

    println!("20.2 {}", grove_value(&sequence, 10));
}
