use std::{fs, collections::HashSet};

struct Games {
    winning: Vec<HashSet<u32>>,
    picked: Vec<HashSet<u32>>,
    copies: Vec<u32>,
    total_games: usize
}

impl Games {
    fn generate(&mut self, input: &String) {
        for line in input.lines() {
            let x = line.find(":").unwrap();
            let y = line.find("|").unwrap();
            let winning_set: HashSet<u32> = line[x+1..y].split(" ").filter(|str| !str.is_empty()).map(|str| str.parse::<u32>().unwrap()).collect();
            let picked_set: HashSet<u32> = line[y+1..].split(" ").filter(|str| !str.is_empty()).map(|str| str.parse::<u32>().unwrap()).collect();
            self.winning.push(winning_set);
            self.picked.push(picked_set);
            self.copies.push(1);
            self.total_games += 1;
        }
    }
    fn part1(&self) {
        let mut output = 0;
        for i in 0..self.total_games {
            let winning_set = &self.winning[i];
            let picked_set = &self.picked[i];
            let number = winning_set.intersection(picked_set).count() as u32;
            if number > 0 {
                output += 2_u32.pow(number - 1);
            }
        }
        println!("{}", output);
    }
    fn part2(&mut self) {
        for i in 0..self.total_games {
            let winning_set = &self.winning[i];
            let picked_set = &self.picked[i];
            let number = winning_set.intersection(picked_set).count();
            for _ in 0..self.copies[i] {
                for k in i+1..(i + number + 1) {
                    *self.copies.get_mut(k).unwrap() += 1;
                }
            }
        }
        let output: u32 = self.copies.iter().sum();
        println!("{}", output);
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Must exist");
    let mut games = Games { 
        winning: vec![],
        picked: vec![],
        copies: vec![],
        total_games: 0,
    };
    games.generate(&input);
    games.part1();
    games.part2();
}

