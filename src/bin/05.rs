advent_of_code::solution!(5);

use once_cell::sync::Lazy;
use regex::Regex;
use std::ops::Range;

fn read_seeds(seeds_row: &str) -> Vec<i64> {
    let seed_id_regex: Regex = Regex::new(r"\d+").unwrap();
    seed_id_regex
        .find_iter(seeds_row)
        .filter_map(|m| m.as_str().parse::<i64>().ok()).collect::<Vec<_>>()
}

fn read_mapping(mapping_row: &str) -> (Range<i64>, i64) {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(
            r"(?<dst>\d+)\s(?<src>\d+)\s(?<length>\d+)"
    ).unwrap());
    let caps = RE.captures(mapping_row).unwrap();
    let source_start = &caps["src"].parse::<i64>().unwrap();
    let destination_start = &caps["dst"].parse::<i64>().unwrap();
    let offset = destination_start - source_start;
    let length = &caps["length"].parse::<i64>().unwrap();
    return (*source_start..source_start+length, offset);
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut sections = input.split_terminator("\n\n");
    let seeds_row = sections.next().unwrap_or("");
    let seeds = read_seeds(&seeds_row);
    let transitions = sections.map(|section| {
        section.split_terminator("\n").skip(1).map(|mapping_row| {
            read_mapping(&mapping_row)
        }).collect::<Vec<(Range<i64>, i64)>>()
    });
    let locations: Vec<i64> = transitions.fold(seeds, |step_vals, step_mappings| {
        step_vals.iter().map(|step_val| {
            match step_mappings.iter()
                .find(|step_mapping| step_mapping.0.contains(&step_val)) {
                    Some((_, offset)) => step_val + offset,
                    _ => *step_val,
                }
        }).collect::<Vec<i64>>()
    });
    let res = u32::try_from(*locations.iter().min().unwrap()).unwrap();
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
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
