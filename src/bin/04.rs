advent_of_code::solution!(4);
use std::collections::HashSet;
use regex::Regex;

fn collect_numbers(number_section: &str, number_pattern: &Regex) -> HashSet<u32> {
    return number_pattern.find_iter(number_section)
        .filter_map(|number_match| number_match.as_str().parse::<u32>().ok())
        .collect::<HashSet<u32>>();
}

pub fn part_one(input: &str) -> Option<u32> {
    let number_pattern = Regex::new(r"\d+").unwrap();
    let res = input.split_terminator("\n").map(|card| {
        let mut numbers_sections = card.split(":").last().unwrap_or("").split("|");
        let winning_numbers_section = &numbers_sections.next().unwrap_or("");
        let drawn_numbers_section = &numbers_sections.last().unwrap_or("");
        let winning_numbers = collect_numbers(&winning_numbers_section, &number_pattern);
        let drawn_numbers = collect_numbers(&drawn_numbers_section, &number_pattern);
        let common_numbers_count = u32::try_from(drawn_numbers.intersection(&winning_numbers).count()).ok();
        match common_numbers_count {
            None => 0,
            Some(0) => 0,
            Some(k) => u32::pow(2, k - 1),
        }
    }).sum::<u32>();
    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
