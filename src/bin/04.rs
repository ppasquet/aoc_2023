advent_of_code::solution!(4);
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug)]
struct Card {
    winning_numbers: HashSet<u32>,
    drawn_numbers: HashSet<u32>,
}

fn build_card(card_details: &str) -> Option<(String, Card)> {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(
            r"Card\s+(?<card_id>\d+):\s(?<winning_nb>[\d\s]*)\s\|\s(?<drawn_nb>[\d\s]*)$").unwrap());
    match RE.captures(card_details) {
        Some(caps) => {
            let winning_numbers = caps["winning_nb"].split_terminator(" ")
                .filter_map(|nb| nb.parse::<u32>().ok()).collect::<HashSet<u32>>();
            let drawn_numbers = caps["drawn_nb"].split_terminator(" ")
                .filter_map(|nb| nb.parse::<u32>().ok()).collect::<HashSet<u32>>();
            Some((String::from(&caps["card_id"]), Card { winning_numbers, drawn_numbers }))
        }
        None => None
    }
}

fn score_card_one(card: &Card) -> u32 {
    let common_numbers_count =
        u32::try_from(card.drawn_numbers.intersection(&card.winning_numbers).count()).ok();
    match common_numbers_count {
        None => 0,
        Some(0) => 0,
        Some(k) => u32::pow(2, k - 1),
    }
}

fn score_card_two(card: &Card) -> u32 {
    let common_numbers_count =
        u32::try_from(card.drawn_numbers.intersection(&card.winning_numbers).count()).ok();
    match common_numbers_count {
        Some(k) => k,
        None => 0,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let res = input
        .split_terminator("\n")
        .map(|row| {
            match build_card(row) {
                Some((_, card)) => score_card_one(&card),
                _ => 0,
            }
        })
        .sum::<u32>();
    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let rows = input.split_terminator("\n");
    let mut card_counts = vec![1; rows.clone().collect::<Vec<_>>().len()];
    for (i, row) in rows.enumerate() {
        let card_matches = match build_card(row) {
            Some((_, card)) => {
                score_card_two(&card)
            },
            None => 0,
        };
        for j in i + 1..i + 1 + card_matches as usize {
            card_counts[j] += card_counts[i];
        }
    }
    Some(card_counts.iter().sum::<u32>())
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
        assert_eq!(result, Some(30));
    }
}
