use nom::error::ErrorKind::TakeWhile1;
use nom::IResult;
use std::fs;

fn first_repeat(input: &str) -> Option<usize> {
    for i in 0..input.len() {
        for j in (i + 1)..input.len() {
            if input[i..=i] == input[j..=j] {
                return Some(i);
            }
        }
    }
    None
}

fn until_n_distinct<const N: usize>(input: &str) -> IResult<&str, &str> {
    let mut index = 0;
    loop {
        if input.len() < N + index {
            return Err(nom::Err::Failure(nom::error::Error::new(input, TakeWhile1)));
        }
        match first_repeat(&input[index..index + N]) {
            Some(x) => index += x + 1,
            None => return Ok((&input[index + N..], &input[..index + N])),
        }
    }
}

fn index_of_n_distinct<const N: usize>(input: &str) -> IResult<&str, usize> {
    nom::combinator::map(until_n_distinct::<N>, |x| x.len())(input)
}

fn main() {
    let input = fs::read_to_string("inputs/day6.txt").unwrap();

    println!("4.1: {}", index_of_n_distinct::<4>(&input).unwrap().1);
    println!("4.2: {}", index_of_n_distinct::<14>(&input).unwrap().1);
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn test_first_repeat() {
        assert_eq!(first_repeat("abbaba"), Some(0));
        assert_eq!(first_repeat("bababa"), Some(0));
        assert_eq!(first_repeat("cabbba"), Some(1));
    }

    #[test]
    fn test_4_repeated() {
        assert_eq!(
            until_n_distinct::<4>("aabbccddefg"),
            Ok(("", "aabbccddefg"))
        );
        assert_eq!(
            until_n_distinct::<4>("aabbccddefghij"),
            Ok(("hij", "aabbccddefg"))
        );
        assert_eq!(
            until_n_distinct::<4>("mjqjpqmgbljsphdztnvjfqwrcgsmlb"),
            Ok(("gbljsphdztnvjfqwrcgsmlb", "mjqjpqm"))
        )
    }
    #[test]
    fn test_part1() {
        let test_input = fs::read_to_string("test_inputs/day6.txt").unwrap();
        let res = test_input
            .lines()
            .filter(|line| !line.is_empty())
            .map(|x| index_of_n_distinct::<4>(x).unwrap().1)
            .collect_vec();
        assert_eq!(res, vec![7, 5, 6, 10, 11]);
    }
    #[test]
    fn test_part2() {
        let test_input = fs::read_to_string("test_inputs/day6.txt").unwrap();
        let res = test_input
            .lines()
            .filter(|line| !line.is_empty())
            .map(|x| index_of_n_distinct::<4>(x).unwrap().1)
            .collect_vec();
        assert_eq!(res, vec![19, 23, 23, 29, 26]);
    }
}
