use std::collections::HashMap;

pub fn process_part1(input: &str) -> u32 {
    input
        .lines()
        .enumerate()
        .map(|(i, line)| match calculate_game_is_possible(line) {
            true => (i + 1) as u32,
            false => 0,
        })
        .sum()
}

pub fn process_part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let power = calculate_fewest_cubes(line);
            println!("power: {}", power);
            power
        })
        .sum()
}

pub fn calculate_game_is_possible(input: &str) -> bool {
    const RED_CUBES: u32 = 12;
    const GREEN_CUBES: u32 = 13;
    const BLUE_CUBES: u32 = 14;

    let input_split: Vec<_> = input.split(":").collect();
    for game in input_split[1].split(";") {
        /* Put the balls into a hashmap */
        let mut balls: HashMap<&str, u32> = HashMap::new();
        for ball in game.split(",") {
            let ball_split: Vec<_> = ball.trim().split(" ").collect();
            balls.insert(ball_split[1], ball_split[0].parse::<u32>().unwrap());
        }

        if balls.contains_key("red") {
            if balls.get("red").unwrap() > &RED_CUBES {
                return false;
            }
        }
        if balls.contains_key("green") {
            if balls.get("green").unwrap() > &GREEN_CUBES {
                return false;
            }
        }
        if balls.contains_key("blue") {
            if balls.get("blue").unwrap() > &BLUE_CUBES {
                return false;
            }
        }
    }

    return true;
}

pub fn calculate_fewest_cubes(input: &str) -> u32 {
    dbg!(input);
    let input_split: Vec<_> = input.split(":").collect();

    let mut red_cubes: Vec<u32> = Vec::new();
    let mut green_cubes: Vec<u32> = Vec::new();
    let mut blue_cubes: Vec<u32> = Vec::new();

    for game in input_split[1].split(";") {
        dbg!(game);

        /* Put the balls into a hashmap */
        let mut balls: HashMap<&str, u32> = HashMap::new();
        for ball in game.split(",") {
            let ball_split: Vec<_> = ball.trim().split(" ").collect();
            balls.insert(ball_split[1], ball_split[0].parse::<u32>().unwrap());
        }

        red_cubes.push(*balls.get("red").unwrap_or(&0));
        green_cubes.push(*balls.get("green").unwrap_or(&0));
        blue_cubes.push(*balls.get("blue").unwrap_or(&0));
    }

    red_cubes.sort();
    green_cubes.sort();
    blue_cubes.sort();

    return red_cubes.last().unwrap() * green_cubes.last().unwrap() * blue_cubes.last().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", true)]
    #[case(
        "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
        true
    )]
    #[case(
        "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
        false
    )]
    #[case(
        "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
        false
    )]
    #[case("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", true)]
    fn game_is_possible(#[case] input: &str, #[case] expected: bool) {
        assert_eq!(calculate_game_is_possible(input), expected);
    }

    #[rstest]
    #[case("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", 48)]
    #[case("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue", 12)]
    #[case(
        "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
        1560
    )]
    #[case(
        "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
        630
    )]
    #[case("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", 36)]
    fn fewest_cubes(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(calculate_fewest_cubes(input), expected);
    }

    const INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    const INPUT2: &str = "";

    #[test]
    fn part1() {
        let result = process_part1(INPUT);
        assert_eq!(result, 8);
    }

    #[test]
    fn part2() {
        let result = process_part2(INPUT);
        assert_eq!(result, 2286);
    }
}
