use std::{collections::HashMap, thread::current, vec};

pub fn process_part1(input: &str) -> u32 {
    /* Get all the numbers, along with their y position, their start x position, and their ending y position */
    dbg!(input);
    let mut numbers: Vec<(u32, u32, u32)> = vec![];
    let mut sum = 0;
    input.lines().enumerate().for_each(|(i, line)| {
        let line_numbers = (get_numbers(line));
        dbg!(&line_numbers);
        for (number, mut start, mut end) in line_numbers {
            dbg!(number, start, end);
            let mut adjacent = false;

            /* Checking for symbols around the current number */
            /* Checking for left and right of number */
            if start != 0 {
                let left_char = line.chars().nth(start as usize - 1).unwrap();
                if (!left_char.is_digit(10)) && (left_char != '.') {
                    sum += number;
                    continue;
                }
            }
            if end != (line.len() - 1) as u32 {
                let right_char = line.chars().nth(end as usize + 1).unwrap();
                if (!right_char.is_digit(10)) && (right_char != '.') {
                    sum += number;
                    continue;
                }
            }

            /* Checking the numbers to above (with the diagonal) */
            if i > 0 {
                let above_line = input.lines().nth(i - 1).unwrap();

                let start_char = match start {
                    0 => 0,
                    _ => start - 1,
                };

                let end_char = match end {
                    x if x == (line.len() - 1) as u32 => x,
                    _ => end + 1,
                };

                for j in (start_char as usize)..=(end_char as usize) {
                    let above_char = above_line.chars().nth(j).unwrap();
                    if (!above_char.is_digit(10)) && (above_char != '.') {
                        sum += number;
                        adjacent = true;
                        break;
                    }
                }
                if adjacent {
                    continue;
                }
            }

            /* Checking the numbers below (with the diagonals) */
            if (i < (input.lines().count() - 1)) {
                let below_line = input.lines().nth(i + 1).unwrap();

                let start_char = match start {
                    0 => 0,
                    _ => start - 1,
                };

                let end_char = match end {
                    x if x == (line.len() - 1) as u32 => x,
                    _ => end + 1,
                };

                for j in (start_char as usize)..=(end_char as usize) {
                    let below_char = below_line.chars().nth(j).unwrap();
                    if (!below_char.is_digit(10)) && (below_char != '.') {
                        sum += number;
                        adjacent = true;
                        break;
                    }
                }
                if adjacent {
                    continue;
                }
            }
        }
    });

    sum
}

pub fn process_part2(input: &str) -> u32 {
    0
}

fn get_numbers(input: &str) -> Vec<(u32, u32, u32)> {
    let mut numbers: Vec<(u32, u32, u32)> = vec![];
    let mut current_number = 0;
    let mut i = 0;
    let mut start = 0;
    let mut end = 0;
    for c in input.chars() {
        if c.is_digit(10) {
            if current_number == 0 {
                start = i;
            }
            current_number *= 10;
            current_number += c.to_digit(10).unwrap();
            if i == ((input.len() - 1) as u32) {
                end = i;
                numbers.push((current_number, start, end));
            }
        } else if current_number != 0 {
            end = i - 1;
            numbers.push((current_number, start, end));
            current_number = 0;
        }
        i += 1;
    }
    numbers
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("467..114..", vec![(467, 0, 2), (114, 5, 7)])]
    #[case("...*......", vec![])]
    #[case("..35..633.", vec![(35, 2, 3), (633, 6, 8)])]
    #[case("......#...", vec![])]
    #[case("617*......", vec![(617, 0, 2)])]
    #[case(".....+.58.", vec![(58, 7, 8)])]
    #[case("..592.....", vec![(592, 2, 4)])]
    #[case("......755.", vec![(755, 6, 8)])]
    #[case("...$.*....", vec![])]
    #[case(".664.598..", vec![(664, 1, 3), (598, 5, 7)])]
    #[case(".664...598", vec![(664, 1, 3), (598, 7, 9)])]
    fn test_get_numbers(#[case] input: &str, #[case] expected: Vec<(u32, u32, u32)>) {
        assert_eq!(get_numbers(input), expected);
    }
    // #[case("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", true)]
    // #[case(
    //     "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
    //     true
    // )]
    // #[case(
    //     "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
    //     false
    // )]
    // #[case(
    //     "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
    //     false
    // )]
    // #[case("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", true)]
    // fn game_is_possible(#[case] input: &str, #[case] expected: bool) {
    //     assert_eq!(calculate_game_is_possible(input), expected);
    // }

    // #[rstest]
    // #[case("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", 48)]
    // #[case("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue", 12)]
    // #[case(
    //     "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
    //     1560
    // )]
    // #[case(
    //     "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
    //     630
    // )]
    // #[case("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", 36)]
    // fn fewest_cubes(#[case] input: &str, #[case] expected: u32) {
    //     assert_eq!(calculate_fewest_cubes(input), expected);
    // }

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

    const INPUT2: &str = "";

    #[test]
    fn part1() {
        let result = process_part1(INPUT);
        assert_eq!(result, 4361);
    }

    #[test]
    fn part2() {
        let result = process_part2(INPUT);
        assert_eq!(result, 2286);
    }
}
