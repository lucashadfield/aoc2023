advent_of_code::solution!(6);

fn count_win_strategies(total_time: i64, record_distance: i64) -> u32 {
    let a = -1;
    let b = total_time;
    let c = -record_distance;

    let root1 =
        (-b as f32 + (b as f32 * b as f32 - 4.0 * a as f32 * c as f32).sqrt()) / (2.0 * a as f32);
    let root2 =
        (-b as f32 - (b as f32 * b as f32 - 4.0 * a as f32 * c as f32).sqrt()) / (2.0 * a as f32);

    (root1.max(root2) - f32::EPSILON * 10 as f32).floor() as u32
        - (root1.min(root2) + f32::EPSILON * 10 as f32).ceil() as u32
        + 1
}

pub fn part_one(input: &str) -> Option<u32> {
    let (total_time, record_distance) = input.trim().split_once('\n').unwrap();

    let (_, times) = total_time.split_once(":").unwrap();
    let (_, distances) = record_distance.split_once(":").unwrap();

    Some(
        times
            .split_whitespace()
            .zip(distances.split_whitespace())
            .map(|(t, d)| (t.parse::<i64>().unwrap(), d.parse::<i64>().unwrap()))
            .map(|(t, d)| count_win_strategies(t, d))
            .product::<u32>(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (total_time, record_distance) = input.trim().split_once('\n').unwrap();

    let (_, times) = total_time.split_once(":").unwrap();
    let (_, distances) = record_distance.split_once(":").unwrap();

    let time = times.replace(" ", "").parse::<i64>().unwrap();
    let distance = distances.replace(" ", "").parse::<i64>().unwrap();

    Some(count_win_strategies(time, distance))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
