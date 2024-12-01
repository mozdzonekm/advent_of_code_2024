use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(1);

fn parse_lists(input: &str) -> (Vec<u32>, Vec<u32>) {
    let (left, right): (Vec<u32>, Vec<u32>) = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .take(2)
                .map(|elem| elem.parse::<u32>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .unzip();
    (left, right)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (left, right) = parse_lists(input);
    let result = left
        .iter()
        .sorted()
        .zip(right.iter().sorted())
        .map(|(a, b)| a.abs_diff(*b))
        .sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (left, right) = parse_lists(input);
    let right_counter = right
        .into_iter()
        .fold(HashMap::<u32, u32>::new(), |mut m, x| {
            *m.entry(x).or_default() += 1;
            m
        });
    let result = left
        .into_iter()
        .map(|x| x * right_counter.get(&x).cloned().unwrap_or(0))
        .sum();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
