use std::{fs, collections::HashMap};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let part1 = part1(&input);
    let part2 = part2(&input);
    println!("part1: {}", part1);
    println!("part2: {}", part2);
}

fn part1(input: &str) -> u32 {
    let mut directions: Vec<char> = vec![];
    let mut nodes: HashMap<&str,(&str, &str)> = HashMap::new();
    let mut key = "AAA";
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        if line.contains('=') {
            let k = &line[0..3];
            let v = match &line[7..15].split(',').collect::<Vec<&str>>()[..] {
                &[x, y, ..] => (x.trim(), y.trim()),
                _ => unreachable!()
            };
            nodes.insert(k, v);
        } else {
            directions = line.chars().collect();
        }
    }
    let mut count = 0;
    let length = directions.len();
    loop {
        if key == "ZZZ" {
            break;
        }
        let value = nodes.get(key).unwrap();
        let direction = &directions[count % length];
        if *direction == 'L' {
            key = value.0;
        } else if *direction == 'R' {
            key = value.1;
        } else {
            unreachable!();
        }
        count += 1;
    }
    count as u32
}

fn part2(input: &str) -> u64 {
    let mut directions: Vec<char> = vec![];
    let mut nodes: HashMap<&str,(&str, &str)> = HashMap::new();
    let mut keys: Vec<&str> = vec![];
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        if line.contains('=') {
            let k = &line[0..3];
            if &k[2..] == "A" {
                keys.push(k);
            }
            let v = match &line[7..15].split(',').collect::<Vec<&str>>()[..] {
                &[x, y, ..] => (x.trim(), y.trim()),
                _ => unreachable!()
            };
            nodes.insert(k, v);
        } else {
            directions = line.chars().collect();
        }
    }
    let mut counts = vec![];
    let length = directions.len();
    for key in keys.iter_mut() {
        let mut count = 0;
        loop {
            if &key[2..] == "Z" {
                break;
            }
            let value = nodes.get(key).unwrap();
            let direction = &directions[count % length];
            if *direction == 'L' {
                *key = value.0;
            } else if *direction == 'R' {
                *key = value.1;
            } else {
                unreachable!();
            }
            count += 1;
        }
        counts.push(count as u64);
    }
    println!("{:?}", counts);
    let mut first = counts.pop().unwrap();
    while let Some(second) = counts.pop() {
        first = lcm(first, second);
    };
    first as u64
}

fn lcm(first: u64, second: u64) -> u64 {
    first * second / gcd(first, second)
}

fn gcd(first: u64, second: u64) -> u64 {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}					

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let test_string1 = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        let test_string2 = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(2_u32, part1(test_string1));
        assert_eq!(6_u32, part1(test_string2));
    }

    #[test]
    fn part2_test() {
        let test_string = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        assert_eq!(6_u64, part2(test_string));
    }
}
