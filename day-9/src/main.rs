use std::{fs, collections::VecDeque};

fn output(input: &String) -> (i64, i64) {
    let mut extrapolations = vec![];
    for line in input.lines() {
        let mut numbers: VecDeque<i64> = line.split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect();
        extrapolations.push(calculate_extrapolation(&mut numbers));
    } 
    extrapolations.iter().fold((0,0), |acc, x| (acc.0 + x.0, acc.1 + x.1))
}

fn calculate_extrapolation(numbers: &mut VecDeque<i64>) -> (i64, i64) {
    let length = numbers.len() - 1;
    let mut count = 1;
    let mut left = numbers.pop_front().unwrap();
    let first = left;
    let mut all_same_diff: Option<(i64, bool)> = None;
    while let Some(right) = numbers.pop_front() {
        let diff = right - left;
        all_same_diff = match all_same_diff {
            Some((old_diff, flag)) => Some((diff, flag && diff - old_diff == 0)),
            None => Some((diff, true))
        };
        numbers.push_back(diff);
        left = right;
        if count >= length {
            break;
        }
        count += 1;
    };
    if all_same_diff.is_some_and(|x| x.1) {
        let diff = all_same_diff.unwrap().0;
        return (left + diff, first - diff);
    }
    let diff = calculate_extrapolation(numbers);
    return (left + diff.0, first - diff.1);
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let output = output(&input);
    println!("part1: {}, part2: {}", output.0, output.1);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn output_test() {
        let test_string = "0 3 6 9 12 15
            1 3 6 10 15 21
            10 13 16 21 30 45".to_string();
        assert_eq!((114_i64, 2_i64), output(&test_string))
    }
}

