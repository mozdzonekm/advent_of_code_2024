advent_of_code::solution!(3);
use regex::Regex;

fn parse(input: &str) -> Vec<(u32, u32)> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut pairs = vec![];
    for (_, [a, b]) in re.captures_iter(input).map(|c| c.extract()) {
        pairs.push((a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap()));
    }
    pairs
}

pub fn part_one(input: &str) -> Option<u32> {
    let result = parse(input).iter().map(|(a, b)| a * b).sum();
    Some(result)
}

fn parse_two(input: &str) -> Vec<(u32, u32)> {
    let mut enabled = true;
    let mul_re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let re = Regex::new(r"(mul\(\d{1,3},\d{1,3}\))|(do\(\))|(don't\(\))").unwrap();
    let mut pairs = vec![];
    for (_, [instruction]) in re.captures_iter(input).map(|c| c.extract()) {
        match instruction {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            _ => {
                let pair = mul_re
                    .captures(instruction)
                    .map(|c| {
                        let (_, [a, b]) = c.extract();
                        (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap())
                    })
                    .unwrap();
                if enabled {
                    pairs.push(pair);
                }
            }
        }
    }
    pairs
}

pub fn part_two(input: &str) -> Option<u32> {
    let result = parse_two(input).iter().map(|(a, b)| a * b).sum();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
