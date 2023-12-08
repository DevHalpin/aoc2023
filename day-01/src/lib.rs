pub fn part1(input: &str) -> String {
    let mut total = 0;
    for line in input.lines() {
        let nums = line
            .chars()
            .filter(|ch| ch.is_ascii_digit())
            .map(|c| c as u8 - b'0')
            .collect::<Vec<u8>>();
        let first = *nums.first().unwrap();
        let last = *nums.last().unwrap();
        total += (first as i32) * 10 + last as i32;
    }
    total.to_string()
}

pub fn part2(input: &str) -> String {
    let mut total = 0;
    let nums_list = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    for line in input.lines() {
        let mut first = None;
        let mut last = None;
        for (i, ch) in line.char_indices() {
            if ch.is_ascii_digit() {
                if first.is_none() {
                    first = Some(ch);
                }
                last = Some(ch);
            }

            for (index, num) in nums_list.iter().enumerate() {
                if i + num.len() <= line.len() && &line[i..i + num.len()] == *num {
                    let digit = (1 + index as u8 + b'0') as char;
                    first = Some(first.unwrap_or(digit));
                    last = Some(digit);
                    break;
                }
            }
        }
        if let (Some(first_digit), Some(last_digit)) = (first, last) {
            let first_value = first_digit.to_digit(10).unwrap();
            let last_value = last_digit.to_digit(10).unwrap();
            total += first_value * 10 + last_value;
        }
    }

    total.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1abc2
  pqr3stu8vwx
  a1b2c3d4e5f
  treb7uchet";

    const INPUT2: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        assert_eq!(result, "142");
    }

    #[test]
    fn part2_works() {
        let result = part2(INPUT2);
        assert_eq!(result, "281");
    }
}
