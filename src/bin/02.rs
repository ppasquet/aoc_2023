advent_of_code::solution!(2);
use regex::{Regex, RegexSet};

const LIMIT: (u32, u32, u32) = (12, 13, 14);
fn find_color_cubes(draw: &str) -> (u32, u32, u32) {
    let red_count_pattern = Regex::new(r"(?<red>\d+) red").unwrap();
    let green_count_pattern = Regex::new(r"(?<green>\d+) green").unwrap();
    let blue_count_pattern = Regex::new(r"(?<blue>\d+) blue").unwrap();
    let red_count = match red_count_pattern.captures(&draw) {
        Some(k) => k["red"].parse().unwrap_or(0),
        _ => 0,
    };
    let green_count = match green_count_pattern.captures(&draw) {
        Some(k) => k["green"].parse().unwrap_or(0),
        _ => 0,
    };
    let blue_count = match blue_count_pattern.captures(&draw) {
        Some(k) => k["blue"].parse().unwrap_or(0),
        _ => 0,
    };
    return (red_count, green_count, blue_count);
}
pub fn part_one(input: &str) -> Option<u32> {
    let game_id_pattern = Regex::new(r"Game (?<game_id>\d+)").unwrap();
    let res = input.split("\n").fold(0, |acc, game| {
        let game_id = match game_id_pattern.captures(&game) {
            Some(caps) => {
                let game_id = caps["game_id"].parse().unwrap_or(0);
                game_id
            },
            _ => 0, // No `Game` prefix
        };
        let game_valid: bool = match game.split(":").last() {
            Some(draws) => {
                draws.split(";").map(|draw| {
                    let (r, g, b) = find_color_cubes(draw);
                    r <= LIMIT.0 && g <= LIMIT.1 && b <= LIMIT.2
                }).all(|valid_draw| valid_draw)
            },
            _ => false, // No game info
        };
        acc + if game_valid { game_id } else { 0 }
    });
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
