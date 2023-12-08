advent_of_code::solution!(1);

use std::collections::HashMap;
// note for self: read the f'ing prompt and examples thoroughly before starting :)
pub fn part_one(input: &str) -> Option<u32> {
    let res: u32 = input.split("\n").fold(0, |acc, cal| {
        // note for self: digits is `mut`-able because its an iterator that updates itself as we
        // progress
        let mut digits = cal.chars().filter_map(|s| s.to_digit(10));
        let t1 = digits.next().unwrap_or(0);
        let t2 = digits.last().unwrap_or(t1);
        acc + (t1 * 10 + t2)
    });
    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let letters_to_digits: HashMap<&str, u32> = HashMap::from([
        ("one", 1),
        ("1", 1),
        ("two", 2),
        ("2", 2),
        ("three", 3),
        ("3", 3),
        ("four", 4),
        ("4", 4),
        ("five", 5),
        ("5", 5),
        ("six", 6),
        ("6", 6),
        ("seven", 7),
        ("7", 7),
        ("eight", 8),
        ("8", 8),
        ("nine", 9),
        ("9", 9)
    ]);
    let res: u32 = input.split("\n").fold(0, |acc, cal| {
        let mut candidates = letters_to_digits.iter().flat_map(|(ft_digit, digit)| {
            cal.match_indices(ft_digit).map(|(i, _)| (i, *digit))
        }).collect::<Vec<_>>();
        candidates.sort();
        let t1 = match candidates.first() {
            Some(i) => i.1,
            _ => 0,
        };
        let t2 = match candidates.last() {
            Some(i) => i.1,
            _ => t1,
        };
        acc + (t1 * 10 + t2)
    });
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(209));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}
