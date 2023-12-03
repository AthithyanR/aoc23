use std::{
    fs,
    collections:: HashMap
};

pub fn main() {
    let number_map = HashMap::from(
        [
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
        ]
    );

    let contents = fs::read_to_string("./src/day1/input").unwrap();
    let lines: Vec<&str> = contents.split("\n").collect();
    let mut calibration_sum = 0;

    for line in &lines {
        let mut nums: Vec<u32> = Vec::new();

        for (idx, c) in line.chars().enumerate() {
            let char_as_digit = c.to_digit(10);
            if char_as_digit.is_some() {
                nums.push(char_as_digit.unwrap())
            } else {
                let line_from_idx = &line[idx..];
                for (k, v) in number_map.iter() {
                    if line_from_idx.starts_with(k) {
                        nums.push(v.clone());
                        continue;
                    }
                }
            }
        } 

        let calibration = nums.first().unwrap() * 10 +  nums.last().unwrap();
        println!("{}{:?}{}", line, nums, calibration);
        calibration_sum += calibration;
    }

    println!("calibration_sum - {}", calibration_sum);
}
