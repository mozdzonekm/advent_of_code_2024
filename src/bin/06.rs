use std::collections::HashSet;

use itertools::Itertools;

advent_of_code::solution!(6);

#[derive(Debug, Hash, PartialEq, Clone, std::cmp::Eq)]
struct Point2D {
    x: i32,
    y: i32,
}

impl Point2D {
    fn transpose(&self, rhs: &Self) -> Self {
        Point2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn position_iside_board(pos: &Point2D, board_x: usize, board_y: usize) -> bool {
    pos.x >= 0 && pos.y >= 0 && pos.x < board_x as i32 && pos.y < board_y as i32
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut obstacles: HashSet<Point2D> = HashSet::new();
    let mut guard_pos: Point2D = Point2D { x: 0, y: 0 };
    let board_x = input.lines().count();
    let board_y = input.lines().collect_vec()[0].chars().count();
    for (x, line) in input.lines().enumerate() {
        for (y, c) in line.chars().enumerate() {
            if c == '#' {
                obstacles.insert(Point2D {
                    x: x as i32,
                    y: y as i32,
                });
            } else if c == '^' {
                guard_pos = Point2D {
                    x: x as i32,
                    y: y as i32,
                };
            }
        }
    }
    let mut direction: Point2D = Point2D { x: -1, y: 0 };
    let mut visited = HashSet::new();
    while position_iside_board(&guard_pos, board_x, board_y) {
        visited.insert(guard_pos.clone());
        let mut next_pos = guard_pos.transpose(&direction);
        while obstacles.contains(&next_pos) {
            direction = match direction {
                Point2D { x: -1, y: 0 } => Point2D { x: 0, y: 1 },
                Point2D { x: 0, y: 1 } => Point2D { x: 1, y: 0 },
                Point2D { x: 1, y: 0 } => Point2D { x: 0, y: -1 },
                Point2D { x: 0, y: -1 } => Point2D { x: -1, y: 0 },
                _ => panic!("Wrong direction"),
            };
            next_pos = guard_pos.transpose(&direction);
        }
        guard_pos = next_pos;
    }
    Some(visited.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
