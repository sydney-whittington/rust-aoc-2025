use itertools::{Itertools};
use nom::{
    IResult, Parser,
    character::complete::{digit1, newline},
    combinator::map,
    multi::separated_list1,
};
use iter_first_max::IterFirstMaxExt as _;

advent_of_code::solution!(3);

fn parser(i: &str) -> IResult<&str, Vec<Vec<u8>>> {
    separated_list1(
        newline,
        map(digit1, |s: &str| {
            s.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>()
        }),
    )
    .parse(i)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (_, batteries) = parser(input).unwrap();

    let mut total: u64 = 0;

    for battery in batteries.iter() {
        let (first_max_index, first_max_value) = battery
            .iter()
            .dropping_back(1) // can't have the max be last or there isn't another digit after
            .enumerate()
            .first_max_by_key(|(_index, joltage)| **joltage)
            .unwrap();

        let second_max_value = battery.iter().skip(first_max_index + 1).max().unwrap();

        println!("for battery {:?}, first max is {}, second max is {}", battery.iter().join(""), first_max_value, second_max_value);
        total += (first_max_value * 10 + second_max_value) as u64;
    }

    Some(total as u64)
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
