pub fn process_part1(input: &str) -> u32 {
    input.lines().map(|line| card_points(line)).sum()
}

pub fn process_part2(input: &str) -> u32 {
    0
}

pub fn card_points(input: &str) -> u32 {
    let winning_numbers = input.split(":").collect::<Vec<_>>()[1]
        .split('|')
        .collect::<Vec<_>>()[0];
    let card_number = input.split(":").collect::<Vec<_>>()[1]
        .split('|')
        .collect::<Vec<_>>()[1];

    let mut sum = 0;

    for number in winning_numbers.split_whitespace() {
        if card_number.contains(number) {
            if sum == 0 {
                sum = 1;
            } else {
                sum *= 2;
            }
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
        assert_eq!(result, 467835);
    }
}
