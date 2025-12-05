use std::collections::HashMap;

use advent_of_code::Coordinate;

advent_of_code::solution!(4);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Warehouse {
    Empty,
    Paper
}

type Region = HashMap<Coordinate<i32>, Warehouse>;

const ADJACENTS: [(i32, i32); 8] = [(-1, 0), (1, 0), (0, -1), (0, 1), (-1, -1), (-1, 1), (1, -1), (1, 1)];

fn parse_region(i: &str) -> Region {
    let mut region = Region::new();
    for (top, line) in i.lines().enumerate() {
        for (left, character) in line.chars().enumerate() {
            match character {
                '@' => {
                    region.insert(Coordinate { left: left as i32, top: top as i32 }, Warehouse::Paper);
                }
                '.' => {
                    // don't actually need to store empty ones if we're not referencing them directly
                    continue;
                    // region.insert(Coordinate { left: left as i32, top: top as i32 }, Warehouse::Empty);
                }
                _ => {
                    panic!("unexpected character");
                }
            }
        }
    }

    region
}
pub fn part_one(input: &str) -> Option<u64> {
    let region = parse_region(input);

    let mut open = 0;

    for paper in region.iter() {
        let mut adjacent = 0;
        for (left, top) in ADJACENTS.iter() {
            let adjacent_coordinate = Coordinate { left: paper.0.left + left, top: paper.0.top + top };
            if let Some(_) = region.get(&adjacent_coordinate) {
                adjacent += 1;
            }
        }
        if adjacent < 4 {
            open += 1;
        }
    }

    Some(open)
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
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
