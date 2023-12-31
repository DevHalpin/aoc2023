#[derive(Debug, Default)]
struct RaceInfo {
    time: i64,
    distance: i64,
}

// MATH!
// time t = total race time
// record r = current record
// button_time = 0..t
// movement_time = t - button_time
// distance = button_time * (t - button_time)
// in order to win, distance must be greater than record
// distance = button_time * (t - button_time) = r + 1
// button_time * (t - button_time) - r - 1 = 0
// button_time^2 - button_time * t - r - 1 = 0

impl RaceInfo {
    fn wins(&self) -> i64 {
        let a = -1_f64;
        let b = self.time as f64;
        let c = (-self.distance - 1) as f64;

        let x1 = ((-b + (b.powi(2) - 4_f64 * a * c).sqrt()) / (2_f64 * a)).ceil();
        let x2 = ((-b - (b.powi(2) - 4_f64 * a * c).sqrt()) / (2_f64 * a)).floor();

        (x2 - x1) as i64 + 1
    }
}

#[derive(Debug, Default)]
struct RaceData {
    races: Vec<RaceInfo>,
    part2race: RaceInfo,
}

fn parse(input: &str) -> RaceData {
    let mut race_data = RaceData::default();
    let lines: Vec<_> = input.lines().collect();
    let times = lines[0]
        .split_once(":")
        .unwrap()
        .1
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect::<Vec<i64>>();
    let distances = lines[1]
        .split_once(":")
        .unwrap()
        .1
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect::<Vec<i64>>();
    for (&time, &distance) in times.iter().zip(distances.iter()) {
        race_data.races.push(RaceInfo { time, distance });
    }
    let part2time = lines[0]
        .split_once(":")
        .unwrap()
        .1
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse::<i64>()
        .unwrap();
    let part2distance = lines[1]
        .split_once(":")
        .unwrap()
        .1
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse::<i64>()
        .unwrap();
    race_data.part2race = RaceInfo {
        time: part2time,
        distance: part2distance,
    };
    println!("race data {:?}", race_data);

    race_data
}

pub fn part1(input: &str) -> String {
    let race_data = parse(input);
    let mut total: i64 = 1;
    for race in race_data.races.iter() {
        total *= race.wins()
    }
    println!("total = {}", total);

    total.to_string()
}

pub fn part2(input: &str) -> String {
    let race_data = parse(input);
    println!("race wins {:?}", race_data.part2race.wins());

    race_data.part2race.wins().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        assert_eq!(result, "288");
    }

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        assert_eq!(result, "71503");
    }
}
