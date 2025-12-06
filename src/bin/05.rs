use std::collections::HashSet;

use itertools::Itertools;
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{newline, u128},
    multi::separated_list1,
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

    Some(fresh)
}

pub fn part_two(input: &str) -> Option<u128> {
    let (_, (ranges, _)) = parser(input).unwrap();
    let ranges = ranges.into_iter().sorted_by_key(|(l, _)| *l).collect_vec();

    let mut valids: HashSet<(u128, u128)> = HashSet::new();
    for (lower, upper) in ranges.iter() {
        if valids.iter().any(|(l, u)| *l <= *lower && *u >= *upper) {
            // introduces no new range, ignore
            continue;
        } else if let Some(mut subsumed) = valids
            .iter()
            .find(|(l, u)| *l <= *lower && *lower <= *u && *u <= *upper)
            .cloned()
        {
            // lower is between and upper is above (or equal)
            valids.remove(&subsumed);
            subsumed.1 = *upper;
            valids.insert(subsumed);
        } else if let Some(mut subsumed) = valids
            .iter()
            .find(|(l, u)| *l >= *lower && *l <= *upper && *u >= *upper)
            .cloned()
        {
            // upper is between and lower is below (or equal)
            valids.remove(&subsumed);
            subsumed.0 = *lower;
            valids.insert(subsumed);
        } else {
            // remove anything entirely contained
            for valid in valids
                .iter()
                .filter(|(l, u)| *l >= *lower && *u <= *upper)
                .cloned()
                .collect::<Vec<_>>()
            {
                valids.remove(&valid);
            }
            valids.insert((*lower, *upper));
        }
    }

    let fresh = valids.iter().map(|(l, u)| u - l + 1).sum();

    Some(fresh)
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
        assert_eq!(result, Some(14));
    }
}
