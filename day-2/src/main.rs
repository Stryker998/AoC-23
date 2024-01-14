use std::fs;
fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    part1(&input);
    part2(&input);
}

fn part1(input: &String) {
    let mut output = 0;
    for line in input.lines() {
        if let Some(x) = line.find(":") {
            let id: u32 = line[5..x].parse().expect("Must be a number");
            let mut num = 0;
            let mut colour_flag = false;
            let mut should_sum = true;
            for c in line[x..].chars() {
                if c.is_numeric() { num = num * 10 + c.to_digit(10).expect("Must be a number"); continue;};
                match c {
                    'b' => { 
                        if num > 14 && !colour_flag { 
                            should_sum = false; 
                            break; 
                        } 
                        colour_flag = true; 
                    }
                    'r' => { 
                        if num > 12 && !colour_flag { 
                            should_sum = false; 
                            break; 
                        } 
                        colour_flag = true; 
                    }
                    'g' => {
                        if num > 13 && !colour_flag {
                            should_sum = false; 
                            break; 
                        } 
                        colour_flag = true; 
                    }
                    ',' => { 
                        num = 0; 
                        colour_flag = false; 
                    }
                    ';' => {
                        num = 0;
                        colour_flag = false; 
                    }
                    _ => {}
                }
            }
            if should_sum {
                output += id;
            }
        }
    }
    println!("{}", output);
}

fn part2(input: &String) {
    let mut output = 0;
    for line in input.lines() {
        if let Some(x) = line.find(":") {
            let mut red_num = 0;
            let mut blue_num = 0;
            let mut green_num = 0;
            let mut num = 0;
            let mut colour_flag = false;
            for c in line[x..].chars() {
                if c.is_numeric() { num = num * 10 + c.to_digit(10).expect("Must be a number"); continue;};
                match c {
                    'b' => { 
                        if blue_num < num && !colour_flag { 
                            blue_num = num;
                        } 
                        colour_flag = true; 
                    }
                    'r' => { 
                        if red_num < num && !colour_flag { 
                            red_num = num;
                        } 
                        colour_flag = true; 
                    }
                    'g' => {
                       if green_num < num && !colour_flag { 
                            green_num = num;
                        } 
                        colour_flag = true; 
                    }
                    ',' => {
                        num = 0;
                        colour_flag = false; 
                    }
                    ';' => {
                        num = 0;
                        colour_flag = false; 
                    }
                    _ => {}
                }
            }
            output += red_num * blue_num * green_num; 
        }
    }
    println!("{}", output);
}
