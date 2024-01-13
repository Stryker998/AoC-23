use std::fs;
fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    part1(&input);
    part2(&input);
}

fn part1(input: &String) {
    let mut output = 0;
    for line in input.lines() {
        let numbers: Vec<u32> = line.chars().filter(|x| x.is_numeric()).map(|x| x.to_digit(10).expect("Must be a number")).collect();
        let num = numbers[0] * 10 + numbers.last().unwrap();
        output += num;
    }
    println!("{}", output);
}

fn part2(input: &String) {
    let mut output = 0;
    let strings = [("one", "1"), ("two", "2"), ("three", "3"), ("four", "4"),("five", "5"), ("six", "6"), ("seven", "7"), ("eight", "8"), ("nine", "9")];
    for line in input.lines() {
        let mut numbers: Vec<(usize, u32)> = line.char_indices().filter(|(_,y)| y.is_numeric()).map(|(x, y)| (x, y.to_digit(10).expect("Must be a number"))).collect(); 
        for (string, number) in strings {
            let temp = line.find(string);
            if let Some(x) = temp {
                numbers.push((x, number.parse().unwrap()));
            }
            let temp1 = line.rfind(string);
            if let Some(y) = temp1 {
                numbers.push((y, number.parse().unwrap()));
            }

        }
        numbers.sort_by(|a,b| a.0.cmp(&b.0));
        let num = numbers.get(0).unwrap_or(&(0,0)).1 * 10 + numbers.last().unwrap_or(&(0,0)).1;
        output += num;
    }
    println!("{}", output);
}
