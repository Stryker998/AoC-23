use std::{fs, collections::VecDeque};

fn get_count(dist: u64, time: u64) -> u64 {
    let mut left = 0;
    let mut right = time;
    let mut i = 0;
    loop {
        if left > right {
            break;
        }
        let total_distance = left * right;
        if total_distance <= dist {
            i += if left == right { 1 } else { 2 };
        } else {
            break;
        }
        left += 1;
        right -= 1;
    }
    return time - i + 1;
}

fn part1(input: &String) -> u64 {
    let mut data = vec![];
    let mut index = 0;
    let mut temp = VecDeque::new();
    for line in input.lines() {
        line.split_whitespace().filter_map(|x| x.parse::<u64>().ok()).for_each(|x| {
            if index == 0 {
                temp.push_back(x);
            } else {
                data.push((temp.pop_front().unwrap(), x));
            }
        });
        index += 1
    }
    let mut result = None;
    for (time, dist) in data.into_iter() {
        let count = get_count(dist, time);
        if count > 0 {
            match result {
                Some(x) => { result = Some(x * count) }
                None => { result = Some(count) }
            }
        }
    }
    return result.unwrap_or(0);
}

fn part2(input: &String) -> u64 {
    let mut time = 0;
    let mut dist = 0;
    let mut index = 0;
    for line in input.lines() {
        if index == 0 {
            time = line[9..].split_whitespace().flat_map(|s| s.chars()).collect::<String>().parse::<u64>().unwrap();
        } else {
            dist = line[9..].split_whitespace().flat_map(|s| s.chars()).collect::<String>().parse::<u64>().unwrap(); 
        }
        index += 1;
    }

    return get_count(dist, time); 
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let part1 = part1(&input);
    let part2 = part2(&input);
    println!("part1: {}", part1);
    println!("part2: {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let test_string = "Time:      7  15   30
Distance:  9  40  200"; 
        let output = part1(&test_string.to_string());
        assert_eq!(288, output);
    } 

    #[test]
    fn test_part2() {
        let test_string = "Time:      7  15   30
Distance:  9  40  200"; 
        let output = part2(&test_string.to_string());
        assert_eq!(71503, output); 
    }
}
