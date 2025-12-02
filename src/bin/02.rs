use itertools::Itertools;
use nom::{
    IResult, Parser, bytes::complete::tag, character::complete::u128, multi::separated_list1,
    sequence::separated_pair,
};

advent_of_code::solution!(2);

fn parser(i: &str) -> IResult<&str, Vec<(u128, u128)>> {
    separated_list1(tag(","), separated_pair(u128, tag("-"), u128)).parse(i)
}

pub fn part_one(input: &str) -> Option<u128> {
    let (_, pairs) = parser(input).unwrap();

    let mut bad = 0;

    for pair in pairs.iter() {
        for n in pair.0..=pair.1 {
            let id = n.to_string();
            // differences in half lengths for odd ones just mean it won't ever be doubled
            let doubled = id[..id.len() / 2] == id[id.len() / 2..];
            if doubled {
                bad += n;
            }
        }
    }

    Some(bad)
}

pub fn part_two(input: &str) -> Option<u128> {
    let (_, pairs) = parser(input).unwrap();

    let mut bad = 0;

    for pair in pairs.iter() {
        for n in pair.0..=pair.1 {
            let id = n.to_string();
            let mut doubled = false;
            for length in 1..=id.len() / 2 {
                doubled |= id
                    .chars()
                    .chunks(length)
                    .into_iter()
                    .map(|c| c.collect::<String>())
                    .all_equal();
            }
            if doubled {
                bad += n;
            }
        }
    }

    Some(bad)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
