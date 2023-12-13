advent_of_code::solution!(9);

fn next_value(values: &Vec<i32>) -> i32 {
    match values.windows(2).map(|w| w[1] - w[0]).collect::<Vec<i32>>() {
        diffs if diffs.iter().all(|v| *v == 0) => *values.iter().last().unwrap(),
        diffs if true => values.iter().last().unwrap() + next_value(&diffs),
        _ => panic!(),
    }
}

fn previous_value(values: &Vec<i32>) -> i32 {
    match values.windows(2).map(|w| w[1] - w[0]).collect::<Vec<i32>>() {
        diffs if diffs.iter().all(|v| *v == 0) => *values.iter().next().unwrap(),
        diffs if true => values.iter().next().unwrap() - previous_value(&diffs),
        _ => panic!(),
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    Some(
        input
            .trim()
            .split('\n')
            .map(|line| {
                line.split(' ')
                    .map(|v| v.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>()
            })
            .map(|values| next_value(&values))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<i32> {
    Some(
        input
            .trim()
            .split('\n')
            .map(|line| {
                line.split(' ')
                    .map(|v| v.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>()
            })
            .map(|values| previous_value(&values))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
