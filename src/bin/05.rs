use std::collections::HashSet;

use itertools::Itertools;
use nom::{
    IResult, Parser, bytes::complete::tag, character::complete::{newline, u128}, multi::separated_list1,
    sequence::separated_pair,
};

advent_of_code::solution!(5);

fn parser(i: &str) -> IResult<&str, (Vec<(u128, u128)>, Vec<u128>)> {
    let (i, ranges) = separated_list1(newline, separated_pair(u128, tag("-"), u128)).parse(i)?;
    let (i, _) = (newline, newline).parse(i)?;
    let (i, ids) = separated_list1(newline, u128).parse(i)?;

    Ok((i, (ranges, ids)))
}

pub fn part_one(input: &str) -> Option<u128> {
    let (_, (ranges, ids)) = parser(input).unwrap();

    let mut fresh = 0;
    for id in ids.iter() {
        if ranges.iter().any(|(l, u)| l <= id && u >= id) {
            fresh += 1;
        }
    }

    Some(fresh as u128)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
