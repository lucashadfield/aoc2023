use itertools::Itertools;
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(4);

fn score_game(line: &str) -> u32 {
    let (winning_cards, candidate_cards) = line.split_once('|').unwrap();

    let winning: HashSet<u32> = winning_cards
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    let candidates: HashSet<u32> = candidate_cards
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    winning
        .intersection(&candidates)
        .cloned()
        .collect::<HashSet<u32>>()
        .len()
        .try_into()
        .unwrap()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input // &str
            .trim()
            .split('\n')
            .map(|line| line.split(':').last().unwrap())
            .map(score_game)
            .map(|s| if s > 0 { u32::pow(2, s - 1) } else { 0 })
            .sum::<u32>(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let base_cards: HashMap<u32, &str> = input
        .trim()
        .split('\n')
        .map(|l| {
            let (card, line) = l.split_once(':').unwrap();
            (
                card.split(' ').last().unwrap().parse::<u32>().unwrap(),
                line,
            )
        })
        .collect();

    let mut card_score: HashMap<u32, u32> = HashMap::new();

    let mut cards: Vec<u32> = base_cards.keys().cloned().sorted().collect();
    let mut i = 0;
    while i < cards.len() {
        let card_num = cards[i];
        let score = card_score
            .entry(card_num)
            .or_insert_with(|| score_game(base_cards.get(&card_num).unwrap()));

        for j in 1..=*score {
            cards.push(cards[i] + j);
        }
        i += 1;
    }

    Some(cards.len().try_into().unwrap())
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
