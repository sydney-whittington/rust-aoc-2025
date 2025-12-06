use itertools::{Itertools, izip};
use nom::{
    IResult, Parser,
    character::complete::{multispace0, one_of, space0, space1, u64},
    multi::{many1, separated_list1},
    sequence::preceded,
};

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Mul,
}

fn operator_from_char(c: &char) -> Operator {
    match c {
        '+' => Operator::Add,
        '*' => Operator::Mul,
        _ => panic!("not an operator"),
    }
}

// https://stackoverflow.com/questions/64498617/how-to-transpose-a-vector-of-vectors-in-rust
fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

advent_of_code::solution!(6);

fn parser1(i: &str) -> IResult<&str, (Vec<Vec<u64>>, Vec<Operator>)> {
    let (i, numbers) = many1(preceded(multispace0, separated_list1(space1, u64))).parse(i)?;
    let (i, operators) = preceded(multispace0, separated_list1(space1, one_of("+*"))).parse(i)?;
    let operators = operators
        .iter()
        .map(|c| operator_from_char(c))
        .collect::<Vec<_>>();

    Ok((i, (numbers, operators)))
}

fn parser2(i: &str) -> IResult<&str, Vec<(Vec<u64>, Operator)>> {
    let (i, problems) = many1((
        preceded(multispace0, separated_list1(space1, u64)),
        preceded(space0, one_of("+*")),
    ))
    .parse(i)?;
    let problems = problems
        .iter()
        .map(|(n, o)| (n.clone(), operator_from_char(o)))
        .collect::<Vec<_>>();

    Ok((i, problems))
}

pub fn part_one(input: &str) -> Option<u64> {
    let (_, math) = parser1(input).unwrap();
    let transposed = transpose(math.0.clone());

    let mut answer = 0;

    for (nums, op) in izip!(transposed.iter(), math.1.iter()) {
        match op {
            Operator::Add => {
                answer += nums.iter().sum::<u64>();
            }
            Operator::Mul => {
                answer += nums.iter().product::<u64>();
            }
        }
    }

    Some(answer)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines: Vec<&str> = input.lines().collect();
    let transposed_lines = transpose(
        lines
            .iter()
            .map(|line| line.chars().rev().collect())
            .collect(),
    );
    let strings = transposed_lines
        .iter()
        .map(|chars| chars.iter().collect::<String>())
        .join("");

    let (_, problems) = parser2(&strings).unwrap();

    let mut answer = 0;
    for (nums, op) in problems.iter() {
        match op {
            Operator::Add => {
                answer += nums.iter().sum::<u64>();
            }
            Operator::Mul => {
                answer += nums.iter().product::<u64>();
            }
        }
    }

    Some(answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
