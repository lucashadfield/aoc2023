use itertools::iproduct;
use itertools::Itertools;
use phf::phf_map;
use std::collections::HashMap;

advent_of_code::solution!(7);

static CARD_RANK_P1: phf::Map<char, u32> = phf_map! {
    '2' => 2,
    '3' => 3,
    '4' => 4,
    '5' => 5,
    '6' => 6,
    '7' => 7,
    '8' => 8,
    '9' => 9,
    'T' => 10,
    'J' => 11,
    'Q' => 12,
    'K' => 13,
    'A' => 14,
};

static CARD_RANK_P2: phf::Map<char, u32> = phf_map! {
    'J' => 1,
    '2' => 2,
    '3' => 3,
    '4' => 4,
    '5' => 5,
    '6' => 6,
    '7' => 7,
    '8' => 8,
    '9' => 9,
    'T' => 10,
    'Q' => 12,
    'K' => 13,
    'A' => 14,
};

struct Hand {
    cards: Vec<char>,
    score: u32,
}

struct RankedHand {
    cards: Vec<char>,
    score: u32,
    rank: usize,
}

fn count_cards(cards: &Vec<char>) -> HashMap<&char, i32> {
    cards.iter().fold(HashMap::new(), |mut acc, card| {
        *acc.entry(card).or_insert(0) += 1;
        acc
    })
}

fn rank(hand_map: HashMap<&char, i32>) -> usize {
    if hand_map.values().any(|&count| count == 5) {
        return 7;
    } else if hand_map.values().any(|&count| count == 4) {
        return 6;
    } else if hand_map.values().any(|&count| count == 3) {
        if hand_map.values().any(|&count| count == 2) {
            return 5;
        } else {
            return 4;
        }
    } else if hand_map.values().any(|&count| count == 2) {
        if hand_map.values().filter(|&&count| count == 2).count() == 2 {
            return 3;
        } else {
            return 2;
        }
    } else {
        return 1;
    }
}

fn simple_hand_rank(hand: &Hand) -> usize {
    rank(count_cards(&hand.cards))
}

fn joker_hand_rank(hand: &Hand) -> usize {
    // unique cards excluding 'J'
    let unique_cards = hand
        .cards
        .iter()
        .filter(|&card| *card != 'J')
        .unique()
        .collect::<Vec<&char>>();

    // fan out any 'J' cards to vec of unique cards
    let fanned_cards: Vec<Vec<&char>> = hand
        .cards
        .iter()
        .map(|c| match c {
            'J' => unique_cards.clone(),
            _ => vec![c],
        })
        .collect();

    // generate all possible hands
    let product = iproduct!(
        fanned_cards[0].iter(),
        fanned_cards[1].iter(),
        fanned_cards[2].iter(),
        fanned_cards[3].iter(),
        fanned_cards[4].iter()
    );

    product
        .map(|(a, b, c, d, e)| vec![**a, **b, **c, **d, **e])
        .map(|h| RankedHand {
            rank: rank(count_cards(&h)),
            cards: hand.cards.clone(),
            score: hand.score,
        })
        .sorted_by(|a, b| compare_hands(a, b, &CARD_RANK_P2))
        .last()
        .unwrap_or(RankedHand {
            // if it's all 'J' then it's 5 of a kind
            rank: 7,
            cards: hand.cards.clone(),
            score: hand.score,
        })
        .rank
}

fn compare_hands(
    a: &RankedHand,
    b: &RankedHand,
    ranker: &phf::Map<char, u32>,
) -> std::cmp::Ordering {
    if a.rank > b.rank {
        return std::cmp::Ordering::Greater;
    } else if a.rank < b.rank {
        return std::cmp::Ordering::Less;
    } else {
        // ranks are equal, compare cards
        for i in 0..a.cards.len() {
            let a_rank = ranker.get(&a.cards[i]).unwrap();
            let b_rank = ranker.get(&b.cards[i]).unwrap();
            if a_rank > b_rank {
                return std::cmp::Ordering::Greater;
            } else if a_rank < b_rank {
                return std::cmp::Ordering::Less;
            }
        }
        return std::cmp::Ordering::Equal;
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .trim()
            .split('\n')
            .map(|line| {
                line.split_once(' ')
                    .map(|(cards, score)| Hand {
                        cards: cards.chars().collect::<Vec<char>>(),
                        score: score.trim().parse::<u32>().unwrap(),
                    })
                    .unwrap()
            })
            .map(|h| RankedHand {
                rank: simple_hand_rank(&h),
                cards: h.cards,
                score: h.score,
            })
            .sorted_by(|a, b| compare_hands(a, b, &CARD_RANK_P1))
            .map(|hand| hand.score)
            .enumerate()
            .map(|(i, score)| score * (i as u32 + 1))
            .sum::<u32>(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .trim()
            .split('\n')
            .map(|line| {
                line.split_once(' ')
                    .map(|(cards, score)| Hand {
                        cards: cards.chars().collect::<Vec<char>>(),
                        score: score.trim().parse::<u32>().unwrap(),
                    })
                    .unwrap()
            })
            .map(|h| RankedHand {
                rank: joker_hand_rank(&h),
                cards: h.cards,
                score: h.score,
            })
            .sorted_by(|a, b| compare_hands(a, b, &CARD_RANK_P2))
            .map(|hand| hand.score)
            .enumerate()
            .map(|(i, score)| score * (i as u32 + 1))
            .sum::<u32>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
