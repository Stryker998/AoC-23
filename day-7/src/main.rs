use std::{fs, collections::{HashMap, BinaryHeap}, cmp::Ordering};

#[derive(PartialEq, Eq, Debug)]
struct Games {
    hand: String,
    bid: u32,
    hand_type: u32,
}

impl PartialOrd for Games {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Games {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Equal => {
                let mut self_iter = self.hand.chars();
                let mut other_iter = other.hand.chars();
                loop {
                    let c1 = self_iter.next().unwrap();
                    let c2 = other_iter.next().unwrap();
                    if c1.is_alphabetic() && c2.is_alphabetic() {
                        let order = ['T', 'J', 'Q', 'K', 'A'];
                        let v1 = order.iter().position(|c| *c == c1).unwrap();
                        let v2 = order.iter().position(|c| *c == c2).unwrap();
                        if v1 > v2 {
                            return Ordering::Greater;
                        } else if v1 < v2 {
                            return Ordering::Less;
                        }
                    } else {
                        if c1 > c2 {
                            return Ordering::Greater;
                        } else if c1 < c2 {
                            return Ordering::Less;
                        }
                    }
                }
            }
            ord => return ord,
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let part1 = part1(&input);
    println!("part1: {}", part1);
    let part2 = part2(&input);
    println!("part2: {}", part2);
}

fn generate(data: &mut Vec<(String, u32)>, input: &String) {
    input.lines().for_each(|line| {
        let input_data: (String, u32) = match &line.split_whitespace().collect::<Vec<&str>>()[..] {
            &[x, y, ..] => (x.to_string(), y.parse::<u32>().unwrap()),
            _ => unreachable!(),
        };
        data.push(input_data);
    });
}

fn part2(input: &String) -> u32 {
    let mut data: Vec<(String, u32)> = vec![];
    generate(&mut data, input);
    let mut output_data = BinaryHeap::new();
    for (hand, bid) in data.into_iter() {
        let mut hand_type = 0;
        let count: HashMap<char, u32> = hand.chars().fold(HashMap::new(), |mut map, c| {
            *map.entry(c).or_insert(0) += 1;
            map
        });
        let mut heap = BinaryHeap::new();
        let mut joker = None;
        count.into_iter().for_each(|x| {
            if x.0 == 'J' { 
                joker = Some(x.1); 
            } else { 
                heap.push(x.1); 
            }
        });
        if let Some(joker_cards) = joker {
            let temp = heap.pop().unwrap_or(0) + joker_cards;
            heap.push(temp);
        }
        let new_hand: String = hand.chars().map(|c| if c == 'J' { '0' } else { c }).collect();
        if let Some(value) = heap.pop() {
            hand_type = match value {
                5 => 7,
                4 => 6,
                3 => if heap.pop().unwrap() == 2 { 5 } else { 4 },
                2 => if heap.pop().unwrap() == 2 { 3 } else { 2 },
                1 => 1,
                _ => unreachable!()
            }
        }
        output_data.push(Games { hand: new_hand, bid, hand_type });
    }
    let mut rank = output_data.len() as u32;
    let mut output = 0;
    while let Some(game) = output_data.pop() {
        output += rank * game.bid;
        rank -= 1;
    }
    output
}

fn part1(input: &String) -> u32 {
    let mut data: Vec<(String, u32)> = vec![];
    generate(&mut data, input);
    let mut output_data = BinaryHeap::new();
    for (hand, bid) in data.into_iter() {
        let mut hand_type = 0;
        let count: HashMap<char, u32> = hand.chars().fold(HashMap::new(), |mut map, c| {
            *map.entry(c).or_insert(0) += 1;
            map
        });
        let mut heap = BinaryHeap::new();
        count.into_iter().for_each(|x| {
            heap.push(x.1);
        });
        if let Some(value) = heap.pop() {
            hand_type = match value {
                5 => 7,
                4 => 6,
                3 => if heap.pop().unwrap() == 2 { 5 } else { 4 },
                2 => if heap.pop().unwrap() == 2 { 3 } else { 2 },
                1 => 1,
                _ => unreachable!()
            }
        }
        output_data.push(Games { hand, bid, hand_type });
    }
    let mut rank = output_data.len() as u32;
    let mut output = 0;
    while let Some(game) = output_data.pop() {
        output += rank * game.bid; 
        rank -= 1;
    }
    output
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_test() {
        let test_string = String::from("32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483");
        let output = part1(&test_string);
        assert_eq!(6440, output);
    }

    #[test]
    fn part2_test() {
        let test_string = String::from("32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483");
        let output = part2(&test_string);
        assert_eq!(5905, output);
    }
}
