use std::collections::VecDeque;

pub fn process_part1(input: &str) -> u32 {
    input.lines().map(|line| card_points(line)).sum()
}

pub fn process_part2(input: &str) -> u32 {
    /* Stack of all the scratch cards */
    /* Originally put all the current cards on the stack */
    let original_cards = input.lines().collect::<Vec<_>>();
    let mut card_stack_usage = vec![0; original_cards.len()];

    let winning_number_vec: Vec<(u33, u32)> = input
        .lines()
        .map(|line| {
            let winning_numbers = line.split(":").collect::<Vec<_>>()[1]
                .split('|')
                .collect::<Vec<_>>()[0];
            let card_number = line.split(":").collect::<Vec<_>>()[1]
                .split('|')
                .collect::<Vec<_>>()[1];

            let card_number: Vec<_> = card_number
                .split_whitespace()
                .map(|number| number.parse::<u32>().unwrap())
                .collect();
            let winning_numbers: Vec<_> = winning_numbers
                .split_whitespace()
                .map(|number| number.parse::<u32>().unwrap())
                .collect();

            let number = line.split(":").collect::<Vec<_>>()[0]
                .split_whitespace()
                .collect::<Vec<_>>()[1]
                .parse::<u32>()
                .unwrap();

            let mut sum = 0;
            for number in card_number {
                if winning_numbers.contains(&number) {
                    // println!("{} is a winning number", number);
                    sum += 1;
                } else {
                    // println!("{} is not a winning number", number);
                }
            }

            (number, sum)
        })
        .collect::<Vec<(u32, u32)>>();

    // dbg!(&winning_number_vec);

    let mut card_stack = winning_number_vec.iter().collect::<VecDeque<&(u32, u32)>>();
    // dbg!(&card_stack);

    // dbg!(&card_stack);

    while card_stack.len() > 0 {
        let card = card_stack.pop_front().unwrap();
        // dbg!(&card);

        let winning_numbers = card.1;
        let card_number = card.0;

        card_stack_usage[card_number as usize - 1] += 1;

        for i in 0..winning_numbers {
            let card_to_push = i + card_number;
            // println!("Pushing card {} to the back", card_to_push);
            let next_card = winning_number_vec[card_to_push as usize];
            let (number, sum) = (next_card.0, next_card.1);
            card_stack.push_back(&winning_number_vec[card_to_push as usize]);
        }
    }

    return card_stack_usage.iter().sum();
}

pub fn card_points(input: &str) -> u32 {
    let winning_numbers = input.split(":").collect::<Vec<_>>()[1]
        .split('|')
        .collect::<Vec<_>>()[0];
    dbg!(&winning_numbers);
    let card_number = input.split(":").collect::<Vec<_>>()[1]
        .split('|')
        .collect::<Vec<_>>()[1];
    dbg!(&card_number);

    let mut sum = 0;

    let card_number: Vec<_> = card_number
        .split_whitespace()
        .map(|number| number.parse::<u32>().unwrap())
        .collect();
    let winning_numbers: Vec<_> = winning_numbers
        .split_whitespace()
        .map(|number| number.parse::<u32>().unwrap())
        .collect();
    dbg!(&card_number);
    dbg!(&winning_numbers);

    for number in card_number {
        if winning_numbers.contains(&number) {
            // println!("{} is a winning number", number);
            if sum == 0 {
                sum = 1;
            } else {
                sum *= 2;
            }
        } else {
            // println!("{} is not a winning number", number);
        }
    }

    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53", 8)]
    #[case("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19", 2)]
    #[case("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1", 2)]
    #[case("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83", 1)]
    #[case("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36", 0)]
    #[case("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11", 0)]
    fn test_card_points(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(card_points(input), expected);
    }

    const INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    const INPUT2: &str = "";

    #[test]
    fn part1() {
        let result = process_part1(INPUT);
        assert_eq!(result, 13);
    }

    #[test]
    fn part2() {
        let result = process_part2(INPUT);
        assert_eq!(result, 30);
    }
}
