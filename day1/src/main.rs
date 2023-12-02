use std::fs;



fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let lines = input.lines();
    
    let mut x: u32 = 0;
    for line in lines.clone() {
        let chars = line.chars();

        let mut first = 0;
        let mut last = 0;
        let mut discovered = false;
        let mut discovered_second = false;
        for char in chars {
            if let Some(digit) = char.to_digit(10) {
                if !discovered {
                    first = digit;
                    discovered = true;
                } else {
                    last = digit;
                    discovered_second = true;
                }
            }
        }
        
        let res = if discovered_second {
            (first * 10) + last
        } else {
            (first * 10) + first
        };

        x += res;
    }
    println!("Part 1: {}", x);

    let nums = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let mut y: u32 = 0;
    for line in lines {
        let mut matches: Vec<(u32, u32)> = Vec::new();

        for num in &nums {
            for index in line.match_indices(num) {
                let res = (index.0.try_into().unwrap(), string_to_digit(index.1).unwrap());
                matches.push(res);
            }
        }

        matches.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        let first = matches.first().unwrap().1;
        let last = matches.last().unwrap().1;

        let res = (first * 10) + last;
        y += res;
    }
    println!("Part 2: {}", y);
}

fn string_to_digit(num: &str) -> Option<u32> {
    match num.to_lowercase().as_str() {
        "one" => Some(1),
        "1" => Some(1),
        "two" => Some(2),
        "2" => Some(2),
        "three" => Some(3),
        "3" => Some(3),
        "four" => Some(4),
        "4" => Some(4),
        "five" => Some(5),
        "5" => Some(5),
        "six" => Some(6),
        "6" => Some(6),
        "seven" => Some(7),
        "7" => Some(7),
        "eight" => Some(8),
        "8" => Some(8),
        "nine" => Some(9),
        "9" => Some(9),
        _ => None,
    }
}
