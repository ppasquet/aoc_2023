advent_of_code::solution!(3);
use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let number_pattern = Regex::new(r"\d+").unwrap();
    let spec_chars_pattern = Regex::new(r"[^0-9\.]").unwrap();
    let rows = std::iter::once(".").chain(input.split_terminator("\n"))
        .chain(std::iter::once(".")).map(|s| String::from(s)).collect::<Vec<String>>();
    let res = rows.windows(3).fold(0, |chunk_acc, chunk| {
        chunk_acc + number_pattern.find_iter(&chunk[1]).fold(0, |row_acc, number_match| {
            let start = if number_match.start() == 0 { 0 } else { number_match.start() - 1 };
            let end = if number_match.end() == chunk[1].len() { number_match.end() } else { number_match.end() + 1 };
            let has_adjacent_spec_char = chunk.iter().map(|row| {
                row.get(start..end)
                    .unwrap_or(row)
            }).any(|x| spec_chars_pattern.is_match(x));
            row_acc + if has_adjacent_spec_char { number_match.as_str().parse().unwrap_or(0) } else { 0 }
        })
    });
    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let gear_char_pattern = Regex::new(r"\*").unwrap();
    let number_pattern = Regex::new(r"\d+").unwrap();
    let rows = std::iter::once(".").chain(input.split_terminator("\n"))
        .chain(std::iter::once(".")).map(|s| String::from(s)).collect::<Vec<String>>();
    let res = rows.windows(3).fold(0, |chunk_acc, chunk| {
        let prod: u32 = gear_char_pattern.find_iter(&chunk[1]).fold(0, |row_acc, gear_match| {
            let terms: Vec<u32> = chunk.iter().map(|row| {
                number_pattern.find_iter(row).filter_map(|number_match| {
                    let start = number_match.start();
                    let end = if number_match.end() == chunk[1].len() { number_match.end() } else { number_match.end() + 1 };
                    let span = start..end;
                    let is_adjacent = span.contains(&gear_match.start()) ||
                        span.contains(&gear_match.end());
                    if is_adjacent { Some(number_match.as_str().parse::<u32>()
                        .unwrap_or(0)) } else { None }
                })
            }).flatten().collect();
            row_acc + if terms.len() == 2 { terms.iter().product() } else { 0 }
        });
        chunk_acc + prod
    });
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
