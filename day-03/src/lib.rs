use std::collections::HashSet;

// Define a struct `PartNumber` to represent a digit with its value and surrounding points
#[derive(Debug)]
struct PartNumber {
    value: i32,
    points: HashSet<(i32, i32)>,
}

// Define a struct `AoCDay3` to hold information about numbers, symbols, and gears on a grid
struct AoCDay3 {
    nums: Vec<PartNumber>,
    syms: HashSet<(i32, i32)>,
    gears: HashSet<(i32, i32)>,
}

impl AoCDay3 {
  // Constructor for creating a new `AoCDay3` instance
  fn new() -> Self {
      Self {
          nums: Vec::new(),
          syms: HashSet::new(),
          gears: HashSet::new(),
      }
  }
}

// Implement methods for the `PartNumber` struct
impl PartNumber {
    // Constructor for creating a new `PartNumber` instance
    fn new(row: i32, col: i32, ch: char) -> Self {
        // Initialize surrounding points based on the provided row and column
        let points = HashSet::from([
          (row - 1, col - 1),
          (row, col - 1),
          (row + 1, col - 1), // values on the left of the number
          (row - 1, col),
          (row + 1, col), // values above and below the number
          (row - 1, col + 1),
          (row, col + 1),
          (row + 1, col + 1), // values on the right of the number
      ]);
        // Create and return a new `PartNumber`
        Self {
            value: ch.to_digit(10).unwrap() as i32,
            points,
        }
    }

    // Method to add a digit to the current digit, updating value and points
    fn add_digit_to_current_digit(&mut self, row: i32, col: i32, ch: char) {
        // Multiply current value by 10 and add the new digit to the end
        self.value = self.value * 10 + (ch.to_digit(10).unwrap() as i32);
        // Extend points to include the next row on the right
        self.points
            .extend([(row - 1, col + 1), (row, col + 1), (row + 1, col + 1)]);
    }
}

pub fn part1(input: &str) -> String {
    // Initialize AoCDay3 struct to store information
    let mut aoc_day3 = AoCDay3::new();
    // Initialize optional current number
    let mut cur_number: Option<PartNumber> = None;
    
    // Iterate through lines and characters in the input
    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            // Check if the character is a digit
            if ch.is_ascii_digit() {
                if let Some(ref mut num) = cur_number {
                    // If there's a current number, add the digit to it
                    num.add_digit_to_current_digit(row as i32, col as i32, ch);
                } else {
                    // If no current number, create a new one
                    cur_number = Some(PartNumber::new(row as i32, col as i32, ch));
                }
            } else {
                if let Some(num) = cur_number.take() {
                    // If the character is not a digit, reset cur_number to None and add the current number to the nums vector
                    aoc_day3.nums.push(num);
                }
                // If the character is not a dot, add its coordinates to the symbols set
                if ch != '.' {
                    aoc_day3.syms.insert((row as i32, col as i32));
                }
            }
        }
    }
    
    // Calculate the total sum based on intersections of number points and symbol points
    let mut total = 0;
    for num in aoc_day3.nums.iter() {
        if num.points.intersection(&aoc_day3.syms).count() != 0 {
            total += num.value;
        }
    }
    total.to_string()
}

// Function to solve part 2 of the Advent of Code challenge
pub fn part2(input: &str) -> String {
    // Initialize AoCDay3 struct to store information
    let mut aoc_day3 = AoCDay3::new();
    // Initialize optional current number
    let mut cur_number: Option<PartNumber> = None;

    // Iterate through lines and characters in the input
    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            // Check if the character is a digit
            if ch.is_ascii_digit() {
                if let Some(ref mut num) = cur_number {
                    // If there's a current number, add the digit to it
                    num.add_digit_to_current_digit(row as i32, col as i32, ch);
                } else {
                    // If no current number, create a new one
                    cur_number = Some(PartNumber::new(row as i32, col as i32, ch));
                }
            } else {
                if let Some(num) = cur_number.take() {
                    // If the character is not a digit, add the current number to the list
                    aoc_day3.nums.push(num);
                }
                // If the character is an asterisk, add its coordinates to the gears set
                if ch == '*' {
                    aoc_day3.gears.insert((row as i32, col as i32));
                }
            }
        }
    }

    // Calculate the total sum based on intersections of gear points and number points
    let mut total = 0;
    for gear in aoc_day3.gears.iter() {
        let mut intersections = Vec::new();
        for num in aoc_day3.nums.iter() {
            if num.points.contains(gear) {
                intersections.push(num.value);
            }
        }
        if intersections.len() == 2 {
            total += intersections[0] * intersections[1];
        }
    }
    total.to_string()
}


#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        assert_eq!(result, "4361");
    }

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        assert_eq!(result, "467835");
    }
}
