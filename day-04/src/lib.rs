#[derive(Debug)]
struct Card {
    winning_numbers: Vec<u32>,
    chosen_numbers: Vec<u32>,
}

fn parse(input: &str) -> Vec<Card> {
    let mut card_results = Vec::new();
    for line in input.lines() {
        let (_, num) = line.split_once(":").unwrap();
        let (winning_numbers, chosen_numbers) = num.split_once("|").unwrap();
        let winning_numbers: Vec<u32> = winning_numbers
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        let chosen_numbers: Vec<u32> = chosen_numbers
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        card_results.push(Card {
            winning_numbers,
            chosen_numbers,
        });
    }
    card_results
}

pub fn part1(input: &str) -> String {
  let cards = parse(input);
  let total = cards.iter().fold(0, |acc, card| {
    let wins_filter = card.chosen_numbers.iter().filter(|&n| card.winning_numbers.contains(n));
    let total_wins = wins_filter.fold((false, 0), |(mut first_match, mut total), &number| {
      if card.winning_numbers.contains(&number) {
        if !first_match {
          first_match = true;
          total += 1;
        } else {
          total *= 2;
        }
      }
      (first_match, total)
    })
    .1;
    acc + total_wins
  });
    total.to_string()
}

pub fn part2(input: &str) -> String {
    let cards = parse(input);
    let multiplier =
        cards
            .iter()
            .enumerate()
            .fold(vec![1usize; cards.len()], |mut acc, (index, card)| {
                let total_wins = card
                    .chosen_numbers
                    .iter()
                    .filter(|&n| card.winning_numbers.contains(n))
                    .count();
                (index + 1..index + 1 + total_wins).for_each(|i| acc[i] += acc[index]);
                acc
            });
    // sum up the total number of copies of each card
    let total = multiplier.iter().sum::<usize>();
    total.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        assert_eq!(result, "13");
    }

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        assert_eq!(result, "30");
    }
}
