use phf::phf_map;
use regex::Regex;
advent_of_code::solution!(1);

static NUMBERS: phf::Map<&'static str, &'static str> = phf_map! {
    "one" => "1",
    "two" => "2",
    "three" => "3",
    "four" => "4",
    "five" => "5",
    "six" => "6",
    "seven" => "7",
    "eight" => "8",
    "nine" => "9",
};

fn parse_line(line: &str) -> u32 {
    let digits = line.chars().filter(|c| c.is_digit(10)).collect::<String>();

    format!(
        "{}{}",
        digits.chars().next().unwrap(),
        digits.chars().last().unwrap()
    )
    .parse::<u32>()
    .unwrap()
}

fn text_to_digits(line: &str) -> String {
    let re = Regex::new(r"(?:one|two|three|four|five|six|seven|eight|nine)").unwrap();

    if let Some(x) = re.find(line) {
        let (start, end) = (x.start(), x.end());
        let matched_string = &line[start..end];
        let number_string = NUMBERS.get(matched_string).unwrap();

        format!(
            "{}{}{}",
            &line[..start],
            number_string,
            text_to_digits(&line[end - 1..])
        )
    } else {
        line.to_string()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.split('\n').map(parse_line).sum::<u32>())
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .split('\n')
            .map(text_to_digits)
            .map(|s| parse_line(s.as_str()))
            .sum::<u32>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}
