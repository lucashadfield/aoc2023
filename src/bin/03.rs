advent_of_code::solution!(3);

// pub fn check_part_number()

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input
        .trim()
        .split('\n')
        .map(|s| s.chars().collect())
        .collect();

    let mut part_num = String::new();
    let mut is_part_adjacent = false;
    let mut part_sum = 0;

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            match check_position(i, j, &grid, &part_num, is_part_adjacent) {
                (Some(num), None) => {
                    part_sum += num;
                    part_num = String::new();
                    is_part_adjacent = false;
                }
                (None, Some(partial)) => {
                    part_num = partial.part_num;
                    is_part_adjacent = partial.is_part_adjacent;
                }
                _ => continue,
            }
        }
    }
    Some(part_sum)
}

fn check_is_part_adjacent(row: usize, col: usize, grid: &Vec<Vec<char>>) -> Option<char> {
    // check if this position has a part around it
    let start_row = row.checked_sub(1).unwrap_or(0);
    let end_row = usize::min(row + 1, grid.len() - 1);
    let start_col = col.checked_sub(1).unwrap_or(0);
    let end_col = usize::min(col + 1, grid.len() - 1);

    for i in start_row..=end_row {
        for j in start_col..=end_col {
            if i != row || j != col {
                match grid[i][j] {
                    p if (!p.is_digit(10)) && (p != '.') => return Some(p),
                    _ => continue,
                }
            }
        }
    }
    None
}

struct PartialFind {
    part_num: String,
    is_part_adjacent: bool,
}

struct Find {
    part_num: Option<u32>,
    symbol: char,
    location: (usize, usize),
    partial_part_num: String,
    is_part_adjacent: bool,
}

fn check_position(
    row: usize,
    col: usize,
    grid: &Vec<Vec<char>>,
    part_num: &String,
    is_part_adjacent: bool,
) -> (Option<u32>, Option<PartialFind>) {
    match grid[row][col] {
        p if p.is_digit(10) => {
            let is_part_adjacent =
                check_is_part_adjacent(row, col, grid).is_some() || is_part_adjacent;

            if (col + 1 < grid[row].len() && !grid[row][col + 1].is_digit(10) && is_part_adjacent)
                || (col + 1 == grid[row].len() && is_part_adjacent)
            {
                // found a valid part
                (
                    Some(format!("{}{}", part_num, p).parse::<u32>().unwrap()),
                    None,
                )
            } else {
                // found a potential partial part
                (
                    None,
                    Some(PartialFind {
                        part_num: format!("{}{}", part_num, p),
                        is_part_adjacent: check_is_part_adjacent(row, col, grid).is_some()
                            || is_part_adjacent,
                    }),
                )
            }
        }
        // found nothing
        _ => (
            None,
            Some(PartialFind {
                part_num: String::new(),
                is_part_adjacent: false,
            }),
        ),
    }
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
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
