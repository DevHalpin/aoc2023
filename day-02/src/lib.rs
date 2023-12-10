fn is_valid_cube(color: &str, count: i32) -> bool {
    println!("color: {:?}, count: {:?}", color, count);
    match color {
        "red" => count <= 12,
        "green" => count <= 13,
        "blue" => count <= 14,
        _ => panic!("invalid color"),
    }
}

pub fn part1(input: &str) -> String {
    let mut total = 0;
    for line in input.lines() {
        let (game, turns) = line.split_once(": ").unwrap();
        let mut valid_turns = 0;
        let turns = turns.split("; ").collect::<Vec<_>>();
        for t in &turns {
            let cube = t.split(", ").collect::<Vec<_>>();
            let mut valid_cubes = 0;
            for c in &cube {
                let (count, color) = c.split_once(" ").unwrap();
                let count = count.parse::<i32>().unwrap();
                if is_valid_cube(color, count) {
                    valid_cubes += 1;
                }
            }
            if valid_cubes == cube.len() {
                valid_turns += 1;
            } 
        }
        if valid_turns == turns.len() {
            let game = game.split_once(" ").unwrap().1;
            total += game.parse::<i32>().unwrap();
        }
    }
    total.to_string()
}

pub fn part2(input: &str) -> String {
    let mut total = 0;
    for line in input.lines() {
      let (_, turns) = line.split_once(": ").unwrap();
      let turns = turns.split("; ").collect::<Vec<_>>();
      let mut red = 0;
      let mut green = 0;
      let mut blue = 0;
      for t in turns {
        let cube = t.split(", ").collect::<Vec<_>>();
        for c in cube {
          let (count, color) = c.split_once(" ").unwrap();
          let count = count.parse::<i32>().unwrap();
          if color == "red" {
            if red < count {
              red = count;
            }
          } else if color == "green" {
            if green < count {
              green = count;
            }
          } else if color == "blue" {
            if blue < count {
              blue = count;
            }
          }
        }
      }
      total += red * green * blue;
    }
    total.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        assert_eq!(result, "8");
    }

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        assert_eq!(result, "2286");
    }
}
