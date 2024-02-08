use std::{fs, collections::HashSet};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let part1 = solution(&input, None);
    let part2 = solution(&input, Some(1000000));
    println!("part1: {}", part1);
    println!("part2: {}", part2);
}

fn solution(input: &String, expand: Option<usize>) -> usize {
    let expand_galaxy: usize = match expand {
        Some(x) => x - 1,
        None => 1,
    };
    let mut row_galaxy = HashSet::new();
    let mut column_galaxy = HashSet::new();
    let mut data = vec![];
    let mut total_row = HashSet::new();
    let mut total_column = HashSet::new();
    for (i, line) in input.lines().enumerate() {
        total_row.insert(i);
        for (j, char) in line.char_indices() {
            total_column.insert(j);
            if char == '#' {
                row_galaxy.insert(i);
                column_galaxy.insert(j);
                data.push((i, j));
            }
        }
    }
    let empty_row: Vec<&usize> = total_row.difference(&row_galaxy).collect();
    let empty_column: Vec<&usize> = total_column.difference(&column_galaxy).collect();
    for (i, j) in data.iter_mut() {
        let mut row_check = 0;
        let mut column_check = 0;
        for row in empty_row.iter() {
            if *i > **row {
                row_check += expand_galaxy;
            }
        }
        for column in empty_column.iter() {
            if *j > **column {
                column_check += expand_galaxy;
            }
        }
        *i += row_check;
        *j += column_check;
    }
    let mut sum = 0;
    for (i, pos1) in data.iter().enumerate() {
        for j in i..data.len() {
            if i == j { continue; }
            let pos2 = &data[j];
            let sum1 = pos1.0.abs_diff(pos2.0) + pos1.1.abs_diff(pos2.1);
            sum += sum1;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let test_string = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....".to_string();
        
        assert_eq!(374_usize, solution(&test_string, None))
    }

    #[test]
    fn part2_test() {
        let test_string = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....".to_string();

        assert_eq!(1030_usize, solution(&test_string, Some(10)));
        assert_eq!(8410_usize, solution(&test_string, Some(100)));
    }
}
