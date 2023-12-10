use std::collections::HashMap;
advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input
        .trim()
        .split('\n')
        .map(|s| s.chars().collect())
        .collect();

    let mut pending_part: Option<PendingPart> = None;
    let mut part_sum = 0;

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            match check_position(i, j, &grid, &pending_part) {
                Some(FindResult::Pending(p)) => {
                    pending_part = Some(p);
                }
                Some(FindResult::Found(p)) => {
                    part_sum += p.part_num;
                    pending_part = None;
                }
                _ => {
                    pending_part = None;
                }
            }
        }
    }
    Some(part_sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input
        .trim()
        .split('\n')
        .map(|s| s.chars().collect())
        .collect();

    let mut pending_part: Option<PendingPart> = None;

    let mut parts_map: HashMap<(usize, usize), Vec<Part>> = HashMap::new();

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            match check_position(i, j, &grid, &pending_part) {
                Some(FindResult::Pending(p)) => {
                    pending_part = Some(p);
                }
                Some(FindResult::Found(p)) => {
                    parts_map.entry(p.location).or_insert_with(Vec::new).push(p);
                    pending_part = None;
                }
                _ => {
                    pending_part = None;
                }
            }
        }
    }

    // filter to locations that have 2 parts
    Some(
        parts_map
            .iter()
            .filter(|(_, v)| v.len() == 2)
            .map(|(_, v)| v[0].part_num * v[1].part_num)
            .sum::<u32>(),
    )
}

#[derive(Clone)]
struct PendingPart {
    is_valid: bool,
    partial_part_num: String,
    symbol: Option<char>,
    location: Option<(usize, usize)>,
}

struct Part {
    part_num: u32,
    symbol: char,
    location: (usize, usize),
}

enum FindResult {
    Found(Part),
    Pending(PendingPart),
}

fn check_is_part_adjacent(row: usize, col: usize, grid: &Vec<Vec<char>>) -> PendingPart {
    // check if this position has a part around it
    let partial_part_num = grid[row][col].to_string();
    let start_row = row.checked_sub(1).unwrap_or(0);
    let end_row = usize::min(row + 1, grid.len() - 1);
    let start_col = col.checked_sub(1).unwrap_or(0);
    let end_col = usize::min(col + 1, grid.len() - 1);

    for i in start_row..=end_row {
        for j in start_col..=end_col {
            if i != row || j != col {
                match grid[i][j] {
                    p if (!p.is_digit(10)) && (p != '.') => {
                        return PendingPart {
                            is_valid: true,
                            partial_part_num: partial_part_num,
                            symbol: Some(p),
                            location: Some((i, j)),
                        };
                    }
                    _ => continue,
                }
            }
        }
    }
    PendingPart {
        is_valid: false,
        partial_part_num: partial_part_num,
        symbol: None,
        location: None,
    }
}

fn merge_pending_parts(first: &PendingPart, second: &PendingPart) -> PendingPart {
    PendingPart {
        is_valid: first.is_valid || second.is_valid,
        partial_part_num: format!("{}{}", first.partial_part_num, second.partial_part_num),
        symbol: first.symbol.or(second.symbol),
        location: first.location.or(second.location),
    }
}

fn check_position(
    row: usize,
    col: usize,
    grid: &Vec<Vec<char>>,
    pending_part: &Option<PendingPart>,
) -> Option<FindResult> {
    match grid[row][col] {
        p if p.is_digit(10) => {
            let mut updated_pending_part: PendingPart;
            if pending_part.as_ref().map_or(false, |pp| pp.is_valid) {
                // no need to check adjacency, just extend the PendingPart
                updated_pending_part = pending_part.as_ref().unwrap().clone();
                updated_pending_part.partial_part_num.push(p)
            } else {
                // there is no pending part, or it isn't yet validated
                let adjacency = check_is_part_adjacent(row, col, grid);
                updated_pending_part = match pending_part {
                    Some(pp) => merge_pending_parts(&pp, &adjacency),
                    None => adjacency,
                };
            }

            // check if it's the last digit
            let terminating_part = (col + 1 == grid[row].len()) || !grid[row][col + 1].is_digit(10);
            if terminating_part {
                if updated_pending_part.is_valid {
                    // if the pending part is valid, convert to Part
                    Some(FindResult::Found(Part {
                        part_num: updated_pending_part
                            .partial_part_num
                            .parse::<u32>()
                            .unwrap(),
                        symbol: updated_pending_part.symbol.unwrap(),
                        location: updated_pending_part.location.unwrap(),
                    }))
                } else {
                    // otherwise, it's not a valid part, return None
                    None
                }
            } else {
                Some(FindResult::Pending(updated_pending_part))
            }
        }
        _ => None,
    }
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
