use std::collections::VecDeque;

use nom::{
    IResult, Parser,
    character::complete::{alpha1, newline, u32},
    multi::separated_list1,
};

advent_of_code::solution!(1);

#[derive(Debug)]
enum Turn {
    R(usize),
    L(usize),
}

fn turn_from_char(c: &str, n: u32) -> Turn {
    match c {
        "R" => Turn::R(n as usize),
        "L" => Turn::L(n as usize),
        _ => panic!("not a turn"),
    }
}

fn parser(i: &str) -> IResult<&str, Vec<Turn>> {
    let (i, a) = separated_list1(newline, (alpha1, u32)).parse(i)?;
    let a = a.iter().map(|(c, n)| turn_from_char(c, *n)).collect();

    Ok((i, a))
}

pub fn part_one(input: &str) -> Option<u64> {
    let (_, turns) = parser(input).unwrap();

    let mut dial = VecDeque::from_iter(0..100);
    dial.rotate_left(50); // initial position per the spec
    let mut password = 0;

    for turn in turns.iter() {
        match turn {
            Turn::R(n) => {
                dial.rotate_right(*n%100);
            }
            Turn::L(n) => {
                dial.rotate_left(*n%100);
            }
        }

        if dial.front() == Some(&0) {
            password += 1;
        }
    }

    Some(password)
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
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
