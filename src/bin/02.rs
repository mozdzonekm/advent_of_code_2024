use std::ops::RangeInclusive;

use itertools::Itertools;

advent_of_code::solution!(2);
const INCRESING: RangeInclusive<i32> = RangeInclusive::new(1, 3);
const DECREASING: RangeInclusive<i32> = RangeInclusive::new(-3, -1);

fn parse_reports(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect_vec()
        })
        .collect_vec()
}
fn is_report_safe(report_increments: &[i32], safe_range: RangeInclusive<i32>) -> bool {
    report_increments.iter().all(|inc| safe_range.contains(inc))
}

fn check_report(report: &[i32]) -> bool {
    let report_increments = report
        .iter()
        .tuple_windows()
        .map(|(current_lvl, next_lvl)| next_lvl - current_lvl)
        .collect_vec();
    is_report_safe(&report_increments, INCRESING) || is_report_safe(&report_increments, DECREASING)
}

pub fn part_one(input: &str) -> Option<u32> {
    let result = parse_reports(input)
        .iter()
        .filter(|report: &&Vec<i32>| check_report(report))
        .count() as u32;
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let result = parse_reports(input)
        .iter()
        .filter(|report: &&Vec<i32>| {
            (0..report.len()).any(|remove_idx: usize| {
                let tmp_report = report
                    .iter()
                    .enumerate()
                    .filter(|&(i, _)| i != remove_idx)
                    .map(|(_, &item)| item)
                    .collect_vec();
                check_report(&tmp_report)
            })
        })
        .count() as u32;
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
