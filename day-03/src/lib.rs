use core::num;
use std::{collections::HashMap, thread::current, vec};

pub fn process_part1(input: &str) -> u32 {
    /* Get all the numbers, along with their y position, their start x position, and their ending y position */
    dbg!(input);
    let mut numbers: Vec<(u32, u32, u32)> = vec![];
    let mut sum = 0;
    input.lines().enumerate().for_each(|(i, line)| {
        let line_numbers = get_numbers(line);
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
            if i < (input.lines().count() - 1) {
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
    let numbers: Vec<_> = input.lines().map(|line| get_numbers(line)).collect();

    let mut sum = 0;
    // println!("{:?}", numbers);

    /* Looping through all the lines, as and looking for a '*' symbol */
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '*' {
                let mut adjacent_number = 0;
                let mut adjacent_sum = 1;
                /* Look around for a number */
                /* Checking to the left */
                if j > 0 {
                    for (number, start, end) in &numbers[i] {
                        if (j as u32) == *end + 1 {
                            // println!("Number to the left {} {} {}", number, start, end);
                            adjacent_number += 1;
                            adjacent_sum *= number;
                            // println!(
                            //     "lef adjacent_number {} adjacent_sum {}",
                            //     adjacent_number, adjacent_sum
                            // );
                            break;
                        }
                    }
                }

                /* Checking to the right */
                if j < (line.len() - 1) {
                    for (number, start, end) in &numbers[i] {
                        if *start != 0 {
                            if (j as u32) == *start - 1 {
                                adjacent_number += 1;
                                adjacent_sum *= number;
                                // println!(
                                //     "right adjacent_number {} adjacent_sum {}",
                                //     adjacent_number, adjacent_sum
                                // );
                                break;
                            }
                        }
                    }
                }

                /* Checking above */
                if i > 0 {
                    for (number, start, end) in &numbers[i - 1] {
                        if (j as u32 >= *start) && (j as u32 <= *end) {
                            adjacent_number += 1;
                            adjacent_sum *= number;
                            // println!("number {} start {} end {}", number, start, end);
                            // println!(
                            //     "Number above adjacent_number {} adjacent_sum {}",
                            //     adjacent_number, adjacent_sum
                            // );
                            break;
                        }

                        /* Checking diagonals */
                        if j > 0 {
                            if (j - 1) as u32 == *end {
                                adjacent_number += 1;
                                adjacent_sum *= number;
                                // println!("number {} start {} end {}", number, start, end);
                                // println!(
                                //     "Number above adjacent_number {} adjacent_sum {}",
                                //     adjacent_number, adjacent_sum
                                // );
                                // break;
                            }
                        }
                        if j < (line.len() - 1) {
                            if (j + 1) as u32 == *start {
                                adjacent_number += 1;
                                adjacent_sum *= number;
                                // println!("number {} start {} end {}", number, start, end);
                                // println!(
                                //     "Number above adjacent_number {} adjacent_sum {}",
                                //     adjacent_number, adjacent_sum
                                // );
                                // break;
                            }
                        }
                    }
                }

                /* Checking below */
                if i < (input.lines().count() - 1) {
                    for (number, start, end) in &numbers[i + 1] {
                        /* Directly below */
                        if (j as u32 >= *start) && (j as u32 <= *end) {
                            adjacent_number += 1;
                            adjacent_sum *= number;
                            println!("number {} start {} end {}", number, start, end);
                            println!(
                                "Number below adjacent_number {} adjacent_sum {}",
                                adjacent_number, adjacent_sum
                            );
                            break;
                        }

                        /* Checking diagonals */
                        if j > 0 {
                            if (j - 1) as u32 == *end {
                                adjacent_number += 1;
                                adjacent_sum *= number;
                                // println!("number {} start {} end {}", number, start, end);
                                // println!(
                                //     "Number above adjacent_number {} adjacent_sum {}",
                                //     adjacent_number, adjacent_sum
                                // );
                                // break;
                            }
                        }
                        if j < (line.len() - 1) {
                            if (j + 1) as u32 == *start {
                                adjacent_number += 1;
                                adjacent_sum *= number;
                                // println!("number {} start {} end {}", number, start, end);
                                // println!(
                                //     "Number above adjacent_number {} adjacent_sum {}",
                                //     adjacent_number, adjacent_sum
                                // );
                                // break;
                            }
                        }
                    }
                }

                if adjacent_number >= 2 {
                    sum += adjacent_sum;
                }
                // println!(
                //     "adjacent_number {} adjacent_sum {}",
                //     adjacent_number, adjacent_sum
                // );
            }
        }
    }

    return sum;
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
        assert_eq!(result, 467835);
    }
}
