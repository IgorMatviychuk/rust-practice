//https://www.hackerrank.com/challenges/kangaroo/problem?isFullScreen=true

use std::io;

fn kangaroo(x1: i32, v1: i32, x2: i32, v2: i32) -> String {
    if v1 <= v2 {
        return "NO".to_string();
    }
    if (x2 - x1) % (v1 - v2) == 0 && (x2 - x1) > 0 {
        return "YES".to_string();
    }
    "NO".to_string()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let nums: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Not an integer"))
        .collect();
    
    let x1 = nums[0];
    let v1 = nums[1];
    let x2 = nums[2];
    let v2 = nums[3];
    
    let result = kangaroo(x1, v1, x2, v2);
    println!("{}", result);
}
