advent_of_code::solution!(5);

struct Category {
    name: String,
    maps: Vec<CategoryMap>,
}

impl Category {
    fn get(&self, source: u64) -> u64 {
        for map in &self.maps {
            match map.get(source) {
                Some(c) => return c,
                None => continue,
            }
        }
        source
    }
}

struct CategoryMap {
    desitination_start: u64,
    source_start: u64,
    length: u64,
}

impl CategoryMap {
    fn get(&self, source: u64) -> Option<u64> {
        if source >= self.source_start && source < self.source_start + self.length {
            Some(source - self.source_start + self.desitination_start)
        } else {
            None
        }
    }
}

fn build_map(input: &str) -> Category {
    let mut maps: Vec<CategoryMap> = Vec::new();
    let (category_name, mappings) = input.split_once('\n').unwrap();
    for line in mappings.trim().split('\n') {
        let parts: Vec<u64> = line.split(' ').map(|s| s.parse::<u64>().unwrap()).collect();
        maps.push(CategoryMap {
            desitination_start: parts[0],
            source_start: parts[1],
            length: parts[2],
        });
    }
    Category {
        name: category_name.to_string(),
        maps,
    }
}

fn closest_seed_location(seeds: Vec<u64>, maps: &Vec<Category>) -> Option<u64> {
    seeds
        .iter()
        .map(|s| {
            let step_1 = maps[0].get(*s);
            let step_2 = maps[1].get(step_1);
            let step_3 = maps[2].get(step_2);
            let step_4 = maps[3].get(step_3);
            let step_5 = maps[4].get(step_4);
            let step_6 = maps[5].get(step_5);
            let step_7 = maps[6].get(step_6);
            step_7
        })
        .min()
}

pub fn part_one(input: &str) -> Option<u64> {
    let (seeds, rest) = input.trim().split_once("\n\n").unwrap();

    // build the maps
    let maps: Vec<Category> = rest.trim().split("\n\n").map(build_map).collect();

    // parse the seeds
    let (_, seeds) = seeds.split_once(':').unwrap();
    let seeds = seeds
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    closest_seed_location(seeds, &maps)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (seeds, rest) = input.trim().split_once("\n\n").unwrap();

    // build the maps
    let maps: Vec<Category> = rest.trim().split("\n\n").map(build_map).collect();

    // parse the seeds
    let (_, seeds) = seeds.split_once(':').unwrap();
    let seed_base: Vec<u64> = seeds
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    let mut seeds: Vec<u64> = Vec::new();
    let mut i = 0;
    while i < seed_base.len() {
        for j in seed_base[i]..seed_base[i] + seed_base[i + 1] {
            seeds.push(j);
        }
        i += 2;
    }

    closest_seed_location(seeds, &maps)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
