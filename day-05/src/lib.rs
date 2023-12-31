use std::{ops::Range, collections::HashSet};

#[derive(Debug, Default)]
struct SeedsAndMaps {
    seeds: Vec<i64>,
    mappings: Vec<VectorOfMaps>,
}

#[derive(Debug, Default)]
struct RangeAndDelta {
    range: Range<i64>,
    delta: i64,
}

#[derive(Debug, Default)]
struct VectorOfMaps {
    map: Vec<RangeAndDelta>,
}

impl VectorOfMaps {
    fn apply_map(&self, current_value: i64) -> i64 {
        for map in &self.map {
            if map.range.contains(&current_value) {
                return current_value + map.delta;
            }
        }
        current_value
    }

    fn reverse_map(&self, current_value: i64) -> i64 {
        for map in &self.map {
            let reverse_value = current_value - map.delta;
            if map.range.contains(&reverse_value) {
                return reverse_value;
            }
        }
        current_value
    }
}

fn parse(input: &str) -> SeedsAndMaps {
    let mut seeds_and_maps = SeedsAndMaps::default();
    let lines: Vec<_> = input.lines().filter(|line| *line != "").collect();
    let seeds = lines[0].split_once(": ").unwrap().1;
    seeds_and_maps.seeds = seeds
        .split(" ")
        .map(|n| n.parse().unwrap())
        .collect();
    let mut current_map = VectorOfMaps::default();
    for line in lines[2..].iter() {
        if line.contains(":") {
            seeds_and_maps.mappings.push(current_map);
            current_map = VectorOfMaps::default();
            continue;
        }
        let numbers: Vec<i64> = line
            .split(" ")
            .map(|n| n.parse().unwrap())
            .collect();
        current_map.map.push(RangeAndDelta {
            range: Range {
                start: numbers[1],
                end: numbers[1] + numbers[2],
            },
            delta: numbers[0] - numbers[1],
        });
    }
    if !current_map.map.is_empty() {
        seeds_and_maps.mappings.push(current_map);
    }
    seeds_and_maps
}

pub fn part1(input: &str) -> String {
    let parsed_input = parse(input);
    let mut min_seed = i64::MAX;
    for seed in &parsed_input.seeds {
        let mut current_seed = *seed;
        for map in &parsed_input.mappings {
            current_seed = map.apply_map(current_seed);
        }
        min_seed = min_seed.min(current_seed);
    }
    min_seed.to_string()
}

pub fn part2(input: &str) -> String {
  let parsed_input = parse(input);
  // BRUTE FORCE
    // let mut min_seed = i64::MAX;
    // for seed_range in parsed_input.seeds.chunks(2) {
    //   for seed in seed_range[0]..seed_range[1] {
    //     let mut current_seed = seed;
    //     for map in &parsed_input.mappings {
    //         current_seed = map.apply_map(current_seed);
    //     }
    //     min_seed = min_seed.min(current_seed);
    //   }
    // }
    // min_seed.to_string()
    // REVERSE LOOKUP
    let seed_ranges_chunks = parsed_input.seeds.chunks(2);
    let seed_ranges: HashSet<Range<i64>> = seed_ranges_chunks
        .map(|seed_range| Range {
            start: seed_range[0],
            end: seed_range[0] + seed_range[1],
        })
        .collect();
    let mut location = 1_i64;
    loop {
      let mut current_value = location;
      for map in parsed_input.mappings.iter().rev() {
          current_value = map.reverse_map(current_value);
      }
      for seed_range in &seed_ranges {
        if seed_range.contains(&current_value) {
          return location.to_string();
        }
      }
      location += 1;
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        assert_eq!(result, "35");
    }

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        assert_eq!(result, "46");
    }
}
