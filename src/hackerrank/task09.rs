//https://www.hackerrank.com/challenges/time-conversion/problem?isFullScreen=true

use std::io;

fn time_conversion(s: &str) -> String {
    let hour = s[0..2].parse::<u32>().unwrap();
    let minute = &s[3..5];
    let second = &s[6..8];
    let period = &s[8..10];

    let hour_24 = match period {
        "AM" if hour == 12 => 0,
        "AM" => hour,
        "PM" if hour != 12 => hour + 12,
        _ => hour,
    };

    format!("{:02}:{minute}:{second}", hour_24)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let trimmed = input.trim();
    let result = time_conversion(trimmed);
    println!("{}", result);
}
