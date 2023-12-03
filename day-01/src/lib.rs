// use regex::Regex;
use fancy_regex::Regex;

pub fn process_part1(input: &str) -> u32 {
    let lines = input.lines();
    let mut sum = 0;
    for line in lines {
        let numbers: Vec<_> = line
            .chars()
            .filter_map(|c| match c.is_numeric() {
                true => Some(c.to_digit(10).unwrap()),
                false => None,
            })
            .collect();
        let number = (numbers.first().unwrap() * 10) + numbers.last().unwrap();
        sum += number;
    }
    sum
}

pub fn process_part2(input: &str) -> u32 {
    const NUMBERS_STR: [&str; 10] = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let lines = input.lines();
    let mut sum = 0;
    for line in lines {
        // println!("line: {}", line);

        let mut v: Vec<u32> = vec![];
        let re_string = "(?=(".to_owned() + &NUMBERS_STR.join("|") + "|\\d))";
        let re = Regex::new(&re_string).unwrap();
        for cap in re.captures_iter(line) {
            // println!("{:?}", cap);
            match cap {
                Ok(c) => {
                    // println!("str: {:?}", c.get(1).unwrap().as_str());
                    let str = c.get(1).unwrap().as_str();
                    if NUMBERS_STR.contains(&str) {
                        // println!("str: {}", str);
                        let index = NUMBERS_STR
                            .iter()
                            .position(|&r| r == str)
                            .expect("No index found");
                        // println!("str: {}", index);
                        v.push(index as u32);
                    } else {
                        // println!("num: {}", str);
                        v.push(str.parse::<u32>().unwrap());
                    }
                }
                Err(e) => {
                    println!("e: {:?}", e);
                }
            }
        }
        // println!("v: {:?}", v);
        sum += (v.first().unwrap() * 10) + v.last().unwrap();
    }
    sum
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
    fn part1() {
        let result = process_part1(INPUT);
        assert_eq!(result, 142);
    }

    #[test]
    fn part2() {
        let result = process_part2(INPUT2);
        assert_eq!(result, 281);
    }
}
