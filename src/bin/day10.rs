use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("inputs/day10.txt")?;

    let instructions = input.lines().map(|line| {
        line.split_once(' ')
            .and_then(|(_, num)| num.parse::<isize>().ok())
    });
    let registers = instructions
        .scan((0, 1), |(pc, x), instruction| {
            match instruction {
                Some(addx) => {
                    *pc += 2;
                    *x += addx
                }
                None => *pc += 1,
            };
            Some((*pc, *x))
        })
        .collect_vec();

    let mut next_pc_value = 20;
    let mut result = 0;
    for (i, (pc, _)) in registers.iter().enumerate() {
        if *pc >= next_pc_value {
            result += next_pc_value * registers[i - 1].1;
            next_pc_value += 40;
        }
    }
    println!("10.1 {result}");

    let mut cycles = registers.into_iter().peekable();
    let mut x = 1_isize;
    let mut display = String::with_capacity(240);
    for pc in 0..=240_isize {
        if let Some((next_pc, _)) = cycles.peek() {
            if *next_pc == pc {
                x = cycles.next().unwrap().1;
            }
        }
        if (pc % 40) == 0 {
            display.push('\n');
        }
        if (pc % 40).abs_diff(x) <= 1 {
            display.push('#');
        } else {
            display.push('.');
        }
    }
    println!("10.2 {display}");
    Ok(())
}
