use phf::phf_map;
use std::collections::HashMap;
advent_of_code::solution!(2);

static GAME_CONFIG: phf::Map<&'static str, u32> = phf_map! {
    "red" => 12,
    "green" => 13,
    "blue" => 14,
};

fn check_game_possible(game: &str) -> Option<u32> {
    let (game_sig, draws) = game.split_once(":").expect("invalid game line");

    let (_, game_id) = game_sig.split_once(" ").expect("invalid game signature");

    let is_valid = draws
        .split(";") // split by count and colour
        .map(|s| s.split(",").map(compare_draw).all(|b| b))
        .all(|b| b);

    if is_valid {
        Some(game_id.parse::<u32>().unwrap())
    } else {
        None
    }
}

fn compare_draw(draw: &str) -> bool {
    let (count, colour) = draw.trim().split_once(" ").expect("invalid draw");
    *GAME_CONFIG.get(colour).unwrap() >= count.parse::<u32>().unwrap()
}

fn calculate_game_power(game: &str) -> u32 {
    let (_, draws) = game.split_once(":").expect("invalid game line");

    draws
        .split(|s| s == ',' || s == ';') // split by count and color
        .map(vectorise_draw) // convert to tuple
        .fold(vec![0, 0, 0], |mut acc, v| {
            acc[0] = acc[0].max(v[0]);
            acc[1] = acc[1].max(v[1]);
            acc[2] = acc[2].max(v[2]);
            acc
        })
        .iter()
        .product()
}

fn vectorise_draw(draw: &str) -> Vec<u32> {
    let (count, colour) = draw.trim().split_once(" ").expect("invalid draw");
    let count = count.parse::<u32>().unwrap();

    match colour {
        "red" => vec![count, 0, 0],
        "green" => vec![0, count, 0],
        "blue" => vec![0, 0, count],
        _ => panic!("invalid colour"),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .trim()
            .split('\n') // one item per game
            .filter_map(check_game_possible) // filter to just game IDs that are valid
            .sum::<u32>(), // sum the valid game IDs
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .trim()
            .split('\n') // one item per game
            .map(calculate_game_power) // filter to just game IDs that are valid
            .sum::<u32>(), // sum the valid game IDs
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
