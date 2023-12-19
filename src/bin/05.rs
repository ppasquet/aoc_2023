advent_of_code::solution!(5);

use once_cell::sync::Lazy;
use regex::Regex;
use std::ops::Range;

fn read_seeds(seeds_row: &str) -> Vec<i64> {
    let seed_id_regex: Regex = Regex::new(r"\d+").unwrap();
    return seed_id_regex
        .find_iter(seeds_row)
        .filter_map(|m| m.as_str().parse::<i64>().ok()).collect::<Vec<_>>();
}

fn read_seed_ranges(seeds_row: &str) -> Vec<Range<i64>> {
    let seed_id_regex: Regex = Regex::new(r"\d+").unwrap();
    let mut seed_ranges = seed_id_regex
        .find_iter(seeds_row)
        .filter_map(|m| m.as_str().parse::<i64>().ok())
        .collect::<Vec<_>>()
        .chunks_exact(2)
        .map(|chunk| {
            chunk[0]..chunk[0]+chunk[1]
        }).collect::<Vec<_>>();
    return seed_ranges;
}

#[derive(Debug)]
struct Mapping {
    range: Range<i64>,
    offset: i64
}

fn read_mapping(mapping_row: &str) -> Mapping {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(
            r"(?<dst>\d+)\s(?<src>\d+)\s(?<length>\d+)"
    ).unwrap());
    let caps = RE.captures(mapping_row).unwrap();
    let source_start = &caps["src"].parse::<i64>().unwrap();
    let destination_start = &caps["dst"].parse::<i64>().unwrap();
    let offset = destination_start - source_start;
    let length = &caps["length"].parse::<i64>().unwrap();
    return Mapping { range: *source_start..source_start+length, offset };
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut sections = input.split_terminator("\n\n");
    let seeds_row = sections.next().unwrap_or("");
    let seeds = read_seeds(&seeds_row);
    let transitions = sections.map(|section| {
        section.split_terminator("\n").skip(1).map(|mapping_row| {
            read_mapping(&mapping_row)
        }).collect::<Vec<Mapping>>()
    });
    let locations: Vec<i64> = transitions.fold(seeds, |step_vals, step_mappings| {
        step_vals.iter().map(|step_val| {
            match step_mappings.iter()
                .find(|step_mapping| step_mapping.range.contains(&step_val)) {
                    Some(m) => step_val + m.offset,
                    _ => *step_val,
                }
        }).collect::<Vec<i64>>()
    });
    let res = u32::try_from(*locations.iter().min().unwrap()).unwrap();
    Some(res)
}

fn find_next_ranges(source: &Range<i64>, transition: &Vec<Mapping>) -> Vec<Range<i64>> {
    if let Some(mapping) = transition.iter()
        .find(|m| m.range.contains(&source.start) && m.range.contains(&(source.end - 1))) {
            // println!("\t{:?} fully contained in {:?}", source, mapping);
            return Vec::from([source.start + mapping.offset..source.end + mapping.offset]);
    }
    if let Some(mapping) = transition.iter()
        .find(|m| m.range.contains(&source.start)) {
            let base = source.start+mapping.offset..mapping.range.end+mapping.offset;
            let remainder = mapping.range.end..source.end;
            let mut next_ranges = Vec::from([base]);
            next_ranges.extend(find_next_ranges(&remainder, transition));
            return next_ranges;
    } else if let Some(mapping) = transition.iter()
        .find(|m| m.range.contains(&(source.end -1 ))) {
            let base = mapping.range.start+mapping.offset..source.end+mapping.offset;
            let remainder = source.start..mapping.range.start;
            let mut next_ranges = Vec::from([base]);
            next_ranges.extend(find_next_ranges(&remainder, transition));
            return next_ranges;
    } else {
        return Vec::from([source.clone()]);
    }
}


pub fn part_two(input: &str) -> Option<u32> {
    let mut sections = input.split_terminator("\n\n");
    let seeds_row = sections.next().unwrap_or("");
    let seed_ranges = read_seed_ranges(&seeds_row);
    let steps = sections.map(|section| {
        let mut foo = section.split_terminator("\n").skip(1).map(|mapping_row| {
            read_mapping(&mapping_row)
        }).collect::<Vec<Mapping>>();
        foo.sort_by_key(|x| x.range.start);
        foo
    });
    let res = seed_ranges.into_iter().filter_map(|seed_range| {
        let locations = steps.clone().fold(Vec::from([seed_range]), |ranges, step| {
            ranges.iter().flat_map(|r| find_next_ranges(r, &step)).collect::<Vec<_>>()
        });
        match locations.iter().min_by_key(|x| x.start) {
            Some(min) => Some(min.start),
            _ => None,
        }
    }).collect::<Vec<_>>();
    match res.iter().min() {
        Some(k) => u32::try_from(*k).ok(),
        _ => None
    }
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
        assert_eq!(result, Some(46));
    }
}
