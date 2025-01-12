use itertools::Itertools;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let board = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let directions = (-1..=1)
        .cartesian_product(-1..=1)
        .filter(|(x, y)| !(*x == 0 && *y == 0))
        .collect_vec();
    let positions = (0..board.len()).cartesian_product(0..board[0].len());
    let result = positions
        .cartesian_product(directions)
        .filter(|((i, j), (diff_i, diff_j))| check_xmas(*i, *j, *diff_i, *diff_j, &board, 0))
        .count() as u32;
    Some(result)
}

fn check_xmas(
    i: usize,
    j: usize,
    diff_i: i32,
    diff_j: i32,
    board: &Vec<Vec<char>>,
    pos: usize,
) -> bool {
    let search_char = match pos {
        0 => 'X',
        1 => 'M',
        2 => 'A',
        3 => 'S',
        _ => panic!("bad position"),
    };
    let search_char_found = board[i][j] == search_char;

    let next_i = i as i32 + diff_i;
    let next_j = j as i32 + diff_j;
    let next_pos_inside_board =
        next_i >= 0 && next_i < board.len() as i32 && next_j >= 0 && next_j < board[0].len() as i32;

    if search_char_found && pos == 3 {
        true
    } else if !search_char_found || !next_pos_inside_board {
        false
    } else {
        check_xmas(
            next_i as usize,
            next_j as usize,
            diff_i,
            diff_j,
            board,
            pos + 1,
        )
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let board = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let result = board
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|(j, c)| {
                    **c == 'A'
                        && i > 0
                        && (i as i32) < (board.len() as i32 - 1)
                        && *j > 0
                        && (*j as i32) < (board.len() as i32) - 1
                })
                .filter(|(j, _)| {
                    (board[i - 1][j - 1] == 'M' && board[i + 1][j + 1] == 'S'
                        || board[i + 1][j + 1] == 'M' && board[i - 1][j - 1] == 'S')
                        && (board[i + 1][j - 1] == 'M' && board[i - 1][j + 1] == 'S'
                            || board[i - 1][j + 1] == 'M' && board[i + 1][j - 1] == 'S')
                })
                .count() as u32
        })
        .reduce(|acc, c| acc + c)
        .unwrap();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
